use std::{sync::{Arc, Mutex}, f32::consts::PI};
use player::{Player, PlayerBuffer};
use threadpool::ThreadPool;
use libnm::{parser::{parse, parse_string}, eval::{eval, default_env}, list::List, program::Item};

pub mod player;

fn main() {


    //let num = our_mutex.lock().unwrap().buffer_size();
    let pool = ThreadPool::new(32);
    let time_mut = Arc::new(Mutex::new(0 as usize));

    let func = match parse_string(format!("(* (sin (* t f)) 0.5)")) {
        Ok(item) => item,
        _ => panic!("Could not parse string!")
    };
    let p: Player = Player::new();

    let buffer_len = p.sample_rate() * 3;
    let mut buffer : Vec<f32> = Vec::new();
    buffer.resize(buffer_len, 0.0);
    let buf_mutex = Arc::new(Mutex::new(buffer));
    for _ in 0..32 {
        let time_c = time_mut.clone();
        let sample_rate = p.sample_rate();
        let buf_mut = buf_mutex.clone();
        let func_clone = func.clone();

        pool.execute(move || {
            loop {
                let mut i = time_c.lock().unwrap();
                if *i % (buffer_len / 10) == 0 {
                    println!("{}%", *i / (buffer_len / 100));
                }

                *i = *i + 1;
                //println!("{}/{}", *i, buffer_len);
                if *i >= buffer_len {
                    break;
                }
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
                let index : usize = *i as usize;
                buf_mut.lock().unwrap()[index] = val;
            }
        })
    }
    pool.join();

    //println!("{}", buf_mutex.lock().unwrap().get(10000).unwrap());

    let mut p = p.build_stream(buf_mutex.lock().unwrap().clone());
    p.play();
    std::thread::sleep(std::time::Duration::from_millis(3000));
    p.pause();
}
