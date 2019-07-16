extern crate glob;
extern crate clap;

use std::{
    process::{exit,Command},
    time::Duration,
    thread::sleep,
    ffi::OsStr,
    option::Option,
    fs
};
use glob::glob;
use clap::{App,Arg};

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn sleep_value_passed_string(){
        assert_eq!(
            Duration::from_millis(120000),
            get_sleep_time(Some("oops"), 2.)
        );
    }
    #[test]
    fn sleep_value_passed_number(){
        assert_eq!(
            Duration::from_millis(60000),
            get_sleep_time(Some("1"), 30.)
        );
    }
    #[test]
    fn match_file_type(){
        let myvar = match Some("String"){
            Some(path) => path,
            None => "Oops"
        };
        assert_eq!("String", myvar);
    }
}


fn get_glob_list(dir_path: &str) -> Vec<String>{
    let mut v:Vec<String> = vec![];
    let query = format!("{}/*.app/Contents/MacOS/*", dir_path);
    for entry in glob(&query).expect("failed to read glob pattern") {
        match entry {
            Ok(path) => v.push(path.display().to_string()),
            Err(e) => println!("{:?}", e),
        }
    }
    return v;
}

fn get_sleep_time(value: Option<&str>, default_time: f32) -> Duration{
    // This feels crazy
    if value.is_some() && value.unwrap().parse::<f32>().is_ok() {
        return Duration::from_millis((value.unwrap().parse::<f32>().unwrap() * 60000.) as u64);
    } else {
        return Duration::from_millis((default_time * 60000.) as u64);
    }
}

fn welcome_message(path: &String, time: &Duration){
    println!("🔄 artloop");
    println!("-----");
    println!("🎨 content path: {}", path);
    println!("⏰ transition time: {:?}ms", time);
    println!("-----");
}

fn delete_ds(path: &String) -> std::io::Result<()>{
    fs::remove_file(format!("{}.DS_STORE", path))?;
    Ok(())
}

fn main() {
    let args = App::new("ARTLOOP")
                    .author("Zach Krall <zachkrall@newschool.edu>")
                    .version("0.1.0")
                    .about("Utility to cycle through a folder of generative art projects.")
                    .arg(Arg::with_name("FOLDER")
                        .help("Selects the folder to use. (Defaults to current folder.)")
                        .required(false)
                    )
                    .arg(Arg::with_name("time")
                        .short("t")
                        .long("time")
                        .help("Specify the duration of each project in minutes.")
                        .takes_value(true))
                    .get_matches();

    let default_time = 10.0;
    let sleep_time = get_sleep_time(args.value_of("time"), default_time);
    let content_path = args.value_of("FOLDER").unwrap_or(".").to_string();
    let apps = get_glob_list(&content_path);

    welcome_message(&content_path, &sleep_time);
<<<<<<< HEAD
=======
    println!("-----");

    let message = delete_ds(&content_path);
    match message {
        Ok(v) => println!("We deleted .DS_Store: {:?}", v),
        Err(e) => println!("we did not delete .DS_Store: {:?}", e),
    }

    v = get_glob_list(&content_path);
>>>>>>> 4b9a1a813dbfdc89d266ffe83fde8210cd67c97d

    if apps.len() < 1 {
        println!("🔍 can't find any apps in {}\n", &content_path);
        println!("please supply a path to a folder containing .app files\n");
        exit(0);
    } if apps.len() == 1 {
        println!("🔍 found 1 app\n-----");
    } else {
        println!("🔍 found {} apps\n!-----", apps.len());
    }

    println!("🛑 to stop artloop, press ctrl+c (^C)\n-----");

    let mut array = apps.iter().cycle();

    loop {
        println!("searching...\n",);
        if array.next().is_some() {
            let file = OsStr::new(array.next().unwrap());
            let mut mycommand = Command::new(file);
            if let Ok(mut child) = mycommand.spawn() {
                println!("... playing \"{:?}\" ...", file);
                sleep(sleep_time);
                child.kill().expect("command expected");
            }
        }
    }
}
