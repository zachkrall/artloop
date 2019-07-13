// use std::{env,fs,process};
// use std::time::{Duration, Instant};
// use std::thread::sleep;

fn min_to_mil(value: f32) -> i32{
    return (value * 60000.) as i32;
}

fn main() {

    // let args: Vec<String> = env::args().collect();
    let sleep_time:f32 = 10.0;

    println!("Hello, world!");

    // for arg in args{
    //     println!("arg: {:?}", arg);
    // }

    println!("{:?} milliseconds", min_to_mil(sleep_time));

    // while true {
    //     let sleep_time = Duration::from_millis(2000);
    //     let now = Instant::now();
    //
    //     sleep(sleep_time);
    //
    //     println!("{} seconds", now.elapsed().as_secs() );
    // }

}
