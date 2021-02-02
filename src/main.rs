extern crate execute;
extern crate regex;
use std::collections::HashMap;
use std::process::Command;
use execute::Execute;
use std::path::Path/*Buf*/;
use std::{env, fs, io};
use clap::{App, Arg};
use regex::Regex;

fn main() {
	let mut files = Vec::new();
	let mut count = 0;
	let mut command_str = "convert".to_string();
	let mut resize = "";
	let mut os = "";
	println!("{:?}", command_str);
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
	//let mut command = String::new();
	//command.push_str("convert");
    //let mut run = Command::new(&command);
    //run.execute_output().unwrap();
    let os = env::consts::OS;
    match os {
    	_ if os == "windows" => { find_binary_windows(); command_str.push_str(".exe"); },
        _ if os == "linux" => find_binary_linux(),
    	_ => println!("0"),
    }
    if matches.is_present("resize") {
    	let re = Regex::new(r"(([\d ]{1,5}[x][\d ]{1,5}))").unwrap();
    	let input = matches.value_of("resize").unwrap();
    	//println!("{:?}", input);
    	if re.is_match(input) {
    		//println!("{:?}", input);
    		resize = input.clone();
    	}
    	else {
    		println!("Invalid resolution provided.");
    		std::process::exit(0);
    	}
    	//resize = input.clone();
    	//println!("resize: {}", resize);
    	//command_str.push_str(" -resize ");
    	//command_str.push_str(input);
    }

    if matches.is_present("dir") {
    	if let Some(ref location) = matches.value_of("dir") {
		    for entry in fs::read_dir(location).unwrap() {
		        let entry = entry.unwrap();
		        let path = entry.path();
		        if path.is_dir() {
		            //println!("{:?} is a dir", path);
		        }
		        else {
		            let re = Regex::new(r"^.*\.(jpg|jpeg|gif|png)$").unwrap();
		            let p = path.clone().into_os_string().into_string().unwrap();
		            if re.is_match(&p) {
		            	println!("matches");
		            	count = count + 1;
		            	files.push(p);
		            }
		        }
		   	}
    	}
    	for i in files {
    		println!("file: {:?}", i);
    	}
    }
    println!("{}", count);
    println!("{:?}", command_str);
    /*for i in files {
    	println!("{:?}", i);
    	if resize.len() >= 1 {
    		convert(command_str.clone(), i.into_os_string().into_string().unwrap(), resize.to_string(), count);
    	}
    }*/
}

fn find_binary_windows() {
	let file = Path::new("C:/Windows/System32/convert.exe").exists();
	if !file {
		println!("ImageMagick not installed, or not installed in expected path(C:/Windows/System32/convert.exe).");
		println!("You can download and install it here: https://imagemagick.org/script/download.php");
		std::process::exit(0);
	}
}

fn find_binary_linux() {
    let file = Path::new("/usr/bin/convert").exists();
    if !file {
        std::process::exit(0);
    }
}

fn convert(command: String, file: String, resize: String, count: i32) {
	// rewrite to be one convert function taking default values
	// test executing
	println!("running convert_resize({}, {}, {})", command, file, resize);
}