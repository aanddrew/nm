use std::sync::{Mutex, Arc};

use cpal::{SampleFormat, Sample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct Player {
    device: cpal::Device,
    config: cpal::StreamConfig,
    stream: Option<cpal::Stream>,

    sample_rate: usize,
    sample_format: cpal::SampleFormat,
}

pub struct PlayerBuffer {
    buffer: Vec<f32>,
    swap: Vec<f32>,
    read_head: usize,
    write_head: usize,
    read_from: BufferReadFrom,
    write_to: BufferReadFrom,
}

unsafe impl Send for PlayerBuffer { }
unsafe impl Send for Player { }

enum BufferReadFrom {
    Buffer,
    Swap
}

impl PlayerBuffer {
    pub fn new(sample_rate: usize) -> Self {
        let mut buffer = Vec::new();
        buffer.resize(sample_rate * 2, 0.0);
        let mut swap = Vec::new();
        swap.resize(sample_rate * 2, 0.0);

        PlayerBuffer {
            read_head: 0,
            write_head: 0,
            buffer: buffer,
            swap: swap,
            read_from: BufferReadFrom::Buffer,
            write_to: BufferReadFrom::Buffer,
        }
    }

    pub fn advance(&mut self) -> f32 {
        let val = match self.read_from {
            BufferReadFrom::Buffer => {
                self.buffer.get(self.read_head).unwrap()
            },
            BufferReadFrom::Swap => {
                self.swap.get(self.read_head).unwrap()
            }
        };
        self.read_head += 1;
        match self.read_from {
            BufferReadFrom::Buffer => {
                if self.read_head >= self.buffer.len() {
                    self.read_head = 0;
                    self.read_from = BufferReadFrom::Swap
                }
            },
            BufferReadFrom::Swap => {
                if self.read_head >= self.buffer.len() {
                    self.read_head = 0;
                    self.read_from = BufferReadFrom::Buffer
                }
            }
        }
        *val
    }

    pub fn write(&mut self, value: f32) {
        match self.write_to {
            BufferReadFrom::Buffer => {
                self.buffer[self.write_head] = value
            },
            BufferReadFrom::Swap => {
                self.swap[self.write_head] = value
            }
        };

        self.write_head += 1;
        match self.write_to {
            BufferReadFrom::Buffer => {
                if self.write_head >= self.buffer.len() {
                    self.write_head = 0;
                    self.write_to = BufferReadFrom::Swap
                }
            },
            BufferReadFrom::Swap => {
                if self.write_head >= self.buffer.len() {
                    self.write_head = 0;
                    self.write_to = BufferReadFrom::Buffer
                }
            }
        }
    }

    pub fn buffer_size(&self ) -> usize{
        self.buffer.len()
    }
}

impl Player {
    pub fn new () -> Self {
        let host = cpal::default_host();

        let device = host.default_output_device().expect("no output device available");

        let mut supported_configs_range = device.supported_output_configs()
            .expect("error while querying configs");
        let supported_config = supported_configs_range.next()
            .expect("no supported config?!")
            .with_max_sample_rate();
        let sample_format = supported_config.sample_format();
        let config : cpal::StreamConfig = supported_config.into();

        let sample_rate = match config.sample_rate {
            cpal::SampleRate(rate) => rate
        };

        Player {
            device: device,
            config: config,
            stream: None,
            sample_rate: sample_rate as usize,
            sample_format,
        }
    }


    pub fn build_stream(mut self, mutex: Arc<Mutex<PlayerBuffer>>) -> Self {
        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

        let mut next_value = move || {
            let mut buffer = mutex.lock().unwrap();
            let res = buffer.advance();
            res
        };
        let channels = self.config.channels;

        //let player = player_mutex.lock().expect("Could not get lock on player in build_strea");
        let stream = match self.sample_format {
            SampleFormat::F32 => {
                self.device.build_output_stream(&self.config, move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    write_data::<f32>(data, channels as usize, &mut next_value)
                }, err_fn) .unwrap()
            },
            SampleFormat::I16 => {
                self.device.build_output_stream(&self.config, move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                    write_data::<i16>(data, channels as usize, &mut next_value)
                }, err_fn) .unwrap()
            },
            SampleFormat::U16 => {
                self.device.build_output_stream(&self.config, move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                    write_data::<u16>(data, channels as usize, &mut next_value)
                }, err_fn) .unwrap()
            },
        };

        fn write_data<T: Sample>(
            data: &mut [T], 
            channels: usize, 
            next_sample: &mut dyn FnMut() -> f32
        ) {
            for frame in data.chunks_mut(channels) {
                let next_value = &next_sample();
                for sample in frame.iter_mut() {
                    //let rand : f32 = rand::thread_rng().gen_range(-1.0..1.0);
                    *sample = Sample::from(next_value);
                }
            }
        }
        self.stream = Some(stream);
        self
    }

    pub fn sample_rate(&self) -> usize {
        self.sample_rate
    }

    pub fn play(&mut self) {
        self.stream.as_ref().expect("Please build stream first").play().unwrap();
    }

    pub fn pause(&mut self) {
        self.stream.as_ref().expect("Please build stream first").pause().unwrap();
    }
}