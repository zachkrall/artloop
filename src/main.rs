use std::time::{Duration, Instant};
use std::thread::sleep;

fn main() {

    println!("Hello, world!");

    while true {
        let ten_millis = Duration::from_millis(2000);
        let now = Instant::now();

        sleep(ten_millis);

        println!("{} seconds", now.elapsed().as_secs() );
    }

}
