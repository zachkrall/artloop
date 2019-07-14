#[allow(dead_code)]
use std::{fs,process};
use std::time::{Duration};
use std::thread::sleep;
use std::ffi::OsStr;

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

// TODO: function to delete ds_store files

fn main() {

    // let args: Vec<String> = env::args().collect();
    let sleep_time = min_to_mil(0.1);

    println!("Hello, world!");

    // for arg in args{
    //     println!("arg: {:?}", arg);
    // }

    let content_path = "./art-loop-content";

    // let mut v = Vec::new();
    let v = get_list(content_path);

    // let paths = fs::read_dir(&content_path).unwrap();
    // for path in paths {
    //     //println!("Name: {}", path.unwrap().path().display())
    //     if let Ok(path) = path {
    //         // Here, `entry` is a `DirEntry`.
    //         let newpath = path.file_name().into_string().unwrap();
    //         let newstem = &path.path().file_stem();
    //         v.push( (newpath, newstem) );
    //     }
    // }

    // let mut iterpath = paths.cycle();

    // let mut array = ["Hello","Goodbye","It's me"].iter().cycle();
    let mut array = v.iter().cycle();

    if v.len() > 0 {
        loop {

            // println!("{:?}",&array.next().unwrap());
            // // println!("{:?}",paths.next().unwrap().path().file_stem().unwrap());
            // sleep(Duration::from_millis(sleep_time));

            // dbg!(&array.next().unwrap());

            let file = &array.next().unwrap();

            let  mypath = format!("./art-loop-content/{}.app/Contents/MacOS/{}", file, file );
            let mut mycommand = process::Command::new(OsStr::new(&mypath));

            if let Ok(mut child) = mycommand.spawn() {
                println!("==process {} begin==", file);
                sleep(Duration::from_millis( sleep_time ));
                child.kill().expect("command wasn't running");
                println!("==process ended==");
                sleep(Duration::from_millis(100));
            } else {
                println!("process unable to start");
            }

        }
    } else {
        println!("Folder is empty");
    }

}
