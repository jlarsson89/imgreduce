//#[macro_use]
//extern crate lazy_static;
extern crate execute;
extern crate regex;
use std::collections::HashMap;
use std::process::Command;
use execute::Execute;
use std::path::Path/*Buf*/;
use std::{env, fs, io};
use clap::{App, Arg};
use regex::Regex;
/*
struct FileCollection { file: File }
struct File { file: Path }
impl FileCollection {
}
lazy_static! {
	static ref HASHMAP: HashMap<u32, &'static str> = {
		let mut m = HashMap::new();
		m
	};
	//static ref INSERT: str = HASHMAP.insert(*COUNT as u32 + 1, str);
	static ref COUNT: usize = HASHMAP.len();
}*/
/*struct FileCollection {
	files: Vec<File>
}*/

/*impl FileCollection {
	pub fn add(f: File) {
		files.push(f)
	}
}*/

/*struct Image {
	file: String
}*/

fn main() {
	let mut files = Vec::new();
	//let mut files = Vec::<&str>::new();
	/*files.push("blah");
	files.push("hlj");
	for i in files.iter() {
		println!("{:?}", i);
	}*/
	let mut count = 0;
	let mut command_str = "convert.exe".to_string();
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
    if matches.is_present("resize") {
    	let _re = Regex::new(r"(([\d ]{1,5}[x][\d ]{1,5}))").unwrap();
    	let input = matches.value_of("resize").unwrap();
    	println!("{:?}", input);
    	command_str.push_str(" -resize ");
    	command_str.push_str(input);
    }

    if matches.is_present("dir") {
    	if let Some(ref location) = matches.value_of("dir") {
	        println!("{}", location);
		    for entry in fs::read_dir(location).unwrap() {
		        let entry = entry.unwrap();
		        let path = entry.path();
		        //let p = path.to_str();
		        //println!("{:?}", p);
		        if path.is_dir() {
		            println!("{:?} is a dir", path);
		        }
		        else {
		            println!("{:?} is a file", path);
		            count = count + 1;
		            let p = path.clone();
		            files.push(p);
		            //files.push(path.display().to_str());
		            /*let mut x = */match path.to_str() {
        				None => panic!("new path is not a valid UTF-8 sequence"),
        				//Some(s) => files.push(s.clone()),
        				Some(s) => println!("new path is {}", s),
    				}//;
    				//files.push(x);
		            //HASHMAP.insert(0, path.to_str();
		        }
		   	}
    	}
    	for i in files {
    		println!("files: {:?}", i);
    	}
    }
    println!("{}", count);
    println!("{:?}", command_str);
    //println!("{:?}", *COUNT);
    /*let current_dir = env::current_dir();
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );*/
}

fn find_binary_windows() {
	let file = Path::new("C:/Windows/System32/convert.exe").exists();
	//println!("{}", file);
	if !file {
		println!("ImageMagick not installed, or not installed in expected path(C:/Windows/System32/convert.exe).");
		println!("You can download and install it here: https://imagemagick.org/script/download.php");
		std::process::exit(0);
	}
}

fn find_binary_linux() {
    let file = Path::new("/usr/bin/convert").exists();
    //println!("{}", file);
    if !file {
        std::process::exit(0);
    }
}
