#[allow(dead_code)]
use std::{
    fs,
    env,
    process,
    time::Duration,
    thread::sleep,
    ffi::OsStr,
    path::Path
};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn convert_time() {
        // Test for time conversion function
        assert_eq!(min_to_mil(10.0), 600000);
    }
}

fn min_to_mil(value: f32) -> u64{
    return (value * 60000.) as u64;
}

fn get_list(action_dir_path: &str) -> Vec<String> {
    fs::read_dir(action_dir_path)
        .unwrap()
        .map(|x| {
            x.unwrap()
                .path()
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string()
        })
        .collect()
}

// TODO:
// fn delete_ds_store(){
//
// }

fn get_args() -> Vec<String>{
    return env::args().collect();
}

fn welcome_message(_args: Vec<String>){
    println!("art loop ...\n");
    println!("passed args:");
    for _arg in _args{
        println!("arg: {}", _arg);
    }
    println!("\n");
}

fn main() {

    let args = get_args();
    let sleep_time = min_to_mil(0.1);
    let content_path = "./art-loop-content";
    let v;

    welcome_message(args);

    if Path::new(content_path).exists() {
        v = get_list(content_path);
    } else {
        // Exit script if content_path does not exist
        println!("\nsupplied path does not exist, bummer.");
        process::exit(0);
    }

    let mut array = v.iter().cycle();

    let mut condition = v.len() > 0;

    while condition {

        println!("\nfinding next artwork ...\n",);
        sleep(Duration::from_millis(1000));

        let file   = &array.next().unwrap();
        let mypath = format!("{}/{}.app/Contents/MacOS/{}", content_path, file, file );

        let mut mycommand = process::Command::new(OsStr::new(&mypath));

        if let Ok(mut child) = mycommand.spawn() {
            println!("started \"{}\" ...", file);
            sleep(Duration::from_millis( sleep_time ));
            child.kill().expect("command wasn't running");
            println!("ended");
        } else {
            condition = false; // break while loop
        }

    }

}
