extern crate glob;
extern crate clap;

use std::{
    process,
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
    fn time_conversion() {
        // Test for time conversion function
        assert_eq!(min_to_mil(10.0), 600000);
    }
    #[test]
    fn sleep_value_passed_string(){
        // Incase clap returns a string from command line prompt
        assert_eq!(2, get_sleep_time(Some("oops"), 2));
    }
}

fn min_to_mil(value: f32) -> u64{
    return (value * 60000.) as u64;
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

fn get_sleep_time(_input: Option<&str>, default_time: u64) -> u64{
    let value;

    if _input == None{
        value = "0.1".parse::<f32>();
    } else {
        value = _input.unwrap().to_string().parse::<f32>();
    }
    if value.is_err() {
        return default_time;
    } else {
        return min_to_mil( value.unwrap() );
    }
}

fn welcome_message(path: &String, time: &u64){
    println!("art loop");
    println!("-----");
    println!("🎨 content path: {}", path);
    println!("⏰ transition time: {}ms", time);
}

fn delete_ds(path: &String) -> std::io::Result<()>{
    fs::remove_file(format!("{}.DS_STORE", path))?;
    Ok(())
}

fn main() {

    let args = App::new("ARTLOOP")
                    .author("Zach Krall <zachkrall@newschool.edu>")
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

    let sleep_time = get_sleep_time(args.value_of("time"), 10);
    let content_path = args.value_of("FOLDER").unwrap_or(".").to_string();

    let v:Vec<String>;

    welcome_message(&content_path, &sleep_time);
    println!("-----");

    let message = delete_ds(&content_path);
    match message {
        Ok(v) => println!("We deleted .DS_Store: {:?}", v),
        Err(e) => println!("we did not delete .DS_Store: {:?}", e),
    }

    v = get_glob_list(&content_path);

    if v.len()<1 {
        println!("🔍 can't find any apps in {}\n", &content_path);
        process::exit(0);
    } else {
        println!("🔍 found {} apps:", v.len());
        for app in &v {
            println!("   * {}", app);
        }
        println!("-----");
    }

    let mut array = v.iter().cycle();

    loop {

        println!("finding next artwork ...\n",);

        let file   = &array.next().unwrap();
        let mut mycommand = process::Command::new(OsStr::new(&file));

        if let Ok(mut child) = mycommand.spawn() {
            println!("started \"{}\" ...", file);
            sleep(Duration::from_millis( sleep_time ));
            child.kill().expect("command wasn't running");
            println!("ended\n");
        }

    }

}
