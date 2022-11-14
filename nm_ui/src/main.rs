use std::{sync::{Arc, Mutex}, f32::consts::PI};
use player::{Player, PlayerBuffer};
use threadpool::ThreadPool;
use libnm::{parser::{parse, parse_string}, eval::{eval, default_env}, list::List, program::Item};

pub mod player;

fn main() {
    let p: Player = Player::new();
    let buf = PlayerBuffer::new(p.sample_rate());
    let buf_mutex = Arc::new(Mutex::new(buf));
    let mut p = p.build_stream(buf_mutex.clone());

    //let num = our_mutex.lock().unwrap().buffer_size();
    let pool = ThreadPool::new(32);
    let time_mut = Arc::new(Mutex::new(0));

    let func = match parse_string(format!("(* (sin (* t f)) 0.5)")) {
        Ok(item) => item,
        _ => panic!("Could not parse string!")
    };

    p.play();
    for _ in 0..1 {
        let time_c = time_mut.clone();
        let sample_rate = p.sample_rate();
        let buf_mut = buf_mutex.clone();
        let func_clone = func.clone();

        pool.execute(move || {
            loop {
                let mut i = time_c.lock().unwrap();
                *i = *i + 1;
                let time = *i as f32 / sample_rate as f32;

                let mut env = default_env();
                env = env.prepend(("f", Item::Float(440.0)));
                env = env.prepend(("t", Item::Float(time)));

                let val = match eval(&func_clone, &env){
                    Ok(Item::Float(val)) => val,
                    Ok(_) => panic!("Error, value is not a float"),
                    Err(msg) => panic!("{}", msg.as_str())
                };

                //while !buf_mut.lock().unwrap().should_write() { }
                buf_mut.lock().unwrap().write(val);
            }
        })
    }
    std::thread::sleep(std::time::Duration::from_millis(3000));
    p.pause();
}
