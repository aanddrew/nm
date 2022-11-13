use std::sync::{Mutex, Arc};

use cpal::{SampleFormat, SampleRate, Sample};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct Player <'a>{
    host: cpal::Host,
    device: cpal::Device,
    config: cpal::StreamConfig,
    stream: Option<cpal::Stream>,

    sample_rate: usize,
    sample_format: cpal::SampleFormat,

    buffer: Arc<Mutex<&'a mut PlayerBuffer>>
}

pub struct PlayerBuffer {
    buffer_size: usize,
    buffer: Vec<f32>,
    swap: Vec<f32>,
    head: usize,
}

unsafe impl Send for PlayerBuffer { }
unsafe impl Send for Player<'_> { }

impl PlayerBuffer {
    pub fn new(sample_rate: usize) -> Self {
        PlayerBuffer {
            head: 0,
            buffer: Vec::new(),
            swap: Vec::new(),
            buffer_size: sample_rate,
        }
    }

    pub fn advance(&mut self) -> f32 {
        self.head += 1;
        0.0
    }
}

impl Player <'_> {
    fn new <'a> (buffer: &'a mut PlayerBuffer) -> Self {
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
            host,
            device,
            config,
            stream: None,
            sample_rate: sample_rate as usize,
            sample_format,

            buffer: Arc::new(Mutex::new(buffer))
        }
    }


    fn build_stream<'a>(&'a mut self) {
        let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

        let mut next_value = move || {
            let mut buffer = self.buffer.lock().unwrap();
            let res = buffer.advance();
            self.buffer.lock().unwrap();
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
        self.stream = Some(stream)
    }
}