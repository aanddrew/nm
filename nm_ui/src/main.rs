use std::{sync::{Arc, Mutex}, f32::consts::PI};
use player::{Player, PlayerBuffer};
use threadpool::ThreadPool;

pub mod player;

fn main() {
    let p: Player = Player::new();
    let buf = PlayerBuffer::new(p.sample_rate());
    let our_mutex = Arc::new(Mutex::new(buf));
    let mut p = p.build_stream(our_mutex.clone());

    let num = our_mutex.lock().unwrap().buffer_size();
    let pool = ThreadPool::new(32);
    let time_mut = Arc::new(Mutex::new(0));

    p.play();
    for _ in 0..32 {
        let time_c = time_mut.clone();
        let sample_rate = p.sample_rate();
        let buf_mut = our_mutex.clone();
        pool.execute(move || {
            loop {
                let mut i = time_c.lock().unwrap();
                *i = *i + 1;
                let sec = *i as f32 / sample_rate as f32;
                buf_mut.lock().unwrap().write((sec * 440.0 * 2.0 * PI).sin() / 2.0);
            }
        })
    }
    std::thread::sleep(std::time::Duration::from_millis(2000));
    p.pause();
}
