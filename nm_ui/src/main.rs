use std::f32::consts::PI;

use cpal::{Sample, SampleFormat, SampleRate};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub mod player;

fn main() {
    let host = cpal::default_host();

    let device = host.default_output_device().expect("no output device available");
    let mut supported_configs_range = device.supported_output_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range.next()
        .expect("no supported config?!")
        .with_max_sample_rate();

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);
    let sample_format = supported_config.sample_format();
    let config: cpal::StreamConfig = supported_config.into();

    let sample_rate = match config.sample_rate {
        SampleRate(rate) => rate
    };

    let mut sample_clock = 0f32;
    let mut next_value = move || {
        sample_clock = sample_clock + 1.0;
        (sample_clock * 440.0 * 2.0 * PI / (sample_rate as f32)).sin()
    };

    let stream = match sample_format {
        SampleFormat::F32 => {
            device.build_output_stream(&config, move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                write_data::<f32>(data, config.channels as usize, &mut next_value)
            }, err_fn) .unwrap()
        },
        SampleFormat::I16 => {
            device.build_output_stream(&config, move |data: &mut [i16], _: &cpal::OutputCallbackInfo| {
                write_data::<i16>(data, config.channels as usize, &mut next_value)
            }, err_fn) .unwrap()
        },
        SampleFormat::U16 => {
            device.build_output_stream(&config, move |data: &mut [u16], _: &cpal::OutputCallbackInfo| {
                write_data::<u16>(data, config.channels as usize, &mut next_value)
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

    stream.play().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(1000));
    stream.pause().unwrap();
}
