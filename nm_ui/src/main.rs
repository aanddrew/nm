use std::{sync::{Arc, Mutex}, f32::consts::PI};

use player::{Player, PlayerBuffer};

pub mod player;

fn main() {
    let p: Player = Player::new();
    let buf = PlayerBuffer::new(p.sample_rate());
    let our_mutex = Arc::new(Mutex::new(buf));
    let mut p = p.build_stream(our_mutex.clone());

    let num = our_mutex.lock().unwrap().buffer_size();


    p.play();
    for i in 0..num {
        let sec = i as f32 / p.sample_rate() as f32;
        our_mutex.lock().unwrap().write((sec * 440.0 * 2.0 * PI).sin() / 2.0);
    }
    std::thread::sleep(std::time::Duration::from_millis(1000));
    p.pause();
}
