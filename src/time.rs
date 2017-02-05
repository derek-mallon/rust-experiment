extern crate time;
use std::time::Duration;
use std::thread;

pub const MAX_FRAME_TIME_NANO :u64 = 16000000;


pub fn do_time(prev:u64)->u64{
    let now = time::precise_time_ns();
    let difference = now - prev;
    if  difference < MAX_FRAME_TIME_NANO{
        thread::sleep(Duration::from_millis((MAX_FRAME_TIME_NANO-difference)/1000000));
    }
    return now;
}
