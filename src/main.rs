extern crate execute;
use std::process::Command;
use execute::Execute;
use std::path::Path/*Buf*/;
use std::{env, fs, io};
use clap::{App, Arg};

fn main() {
	let matches = App::new("imgreduce")
		.arg(
			Arg::with_name("dir")
			.help("Directory")
			.takes_value(true)
			.required(true)
			.short("d")
			)
		.arg(
			Arg::with_name("resize")
			.help("Resolution to resize to")
			.takes_value(true)
			.short("r")
			)
		.arg(
			Arg::with_name("format")
			.help("Convert into a new file format")
			.takes_value(true)
			.possible_values(&[".jpg", ".jpeg", ".gif", ".png"])
			.short("f")
			)
		.arg(
			Arg::with_name("pretty")
			.help("Display information")
			.takes_value(false)
			.short("p")
			)
		.get_matches();
	let mut command = String::new();
	command.push_str("convert");
//    let result = which::which("convert").unwrap();
    //assert_eq!(result, PathBuf::from("/usr/bin/convert"));
    //let mut run = Command::new(&command);
    //run.execute_output().unwrap();
    let os = env::consts::OS;
    match os {
    	_ if os == "windows" => find_binary_windows(),
        _ if os == "linux" => find_binary_linux(),
    	_ => println!("0"),
    }
    if matches.is_present("dir") {
    	if let Some(ref location) = matches.value_of("dir") {
        println!("{}", location);
	    for entry in fs::read_dir(location).unwrap() {
	        let entry = entry.unwrap();
	        let path = entry.path();
	        if path.is_dir() {
	            println!("{:?} is a dir", path);
	        } else {
	            println!("{:?} is a file", path);
	        }
	    }
    }
    }
    /*let current_dir = env::current_dir();
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );*/
}

fn find_binary_windows() {
	let file = Path::new("C:\\Windows\\System32\\convert.exe").exists();
	println!("{}", file);
	if !file {
		std::process::exit(0);
	}
}

fn find_binary_linux() {
    let file = Path::new("/usr/bin/convert").exists();
    println!("{}", file);
    if !file {
        std::process::exit(0);
    }
}
