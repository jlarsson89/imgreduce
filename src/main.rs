extern crate regex;
use std::process::Command;
use std::path::Path;
use std::{env, fs};
use clap::{App, Arg};
use regex::Regex;

fn main() {
   	let mut files = Vec::new();
	let mut count = 0;
	let mut command_str = "".to_string();
	let mut resize = "";
	let mut pretty = false;
	let mut format = "";
	let file_format = Regex::new(r"^.*\.(?i)(jpg|jpeg|gif|png)$").unwrap();
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
    let os = env::consts::OS;
    match os {
    	_ if os == "windows" => { command_str = find_binary_windows(); },
        _ if os == "linux" => { command_str = find_binary_linux(); },
    	_ => println!("0"),
    }
    if matches.is_present("resize") {
    	let re = Regex::new(r"(([\d ]{1,5}[x][\d ]{1,5}))").unwrap();
    	let input = matches.value_of("resize").unwrap();
    	if re.is_match(input) {
    		resize = input.clone();
    	}
    	else {
    		println!("Invalid resolution provided.");
    		std::process::exit(0);
    	}
    }
    if matches.is_present("dir") {
    	if let Some(ref location) = matches.value_of("dir") {
		    for entry in fs::read_dir(location).unwrap() {
		        let entry = entry.unwrap();
		        let path = entry.path();
		        if path.is_dir() {
		        }
		        else {
		            let p = path.clone().into_os_string().into_string().unwrap();
		            if file_format.is_match(&p) {
		            	if os == "windows" {
		            		let np = p.replace(r"\", "/");
		            		count = count + 1;
		            		files.push(np);
		            	}
		            	else {
		            		count = count + 1;
		            		files.push(p);
		            	}
		            }
		        }
		   	}
    	}
    }
    if matches.is_present("pretty") {
    	pretty = true;
    }
    if matches.is_present("format") {
    	let input = matches.value_of("format").unwrap();
    	if file_format.is_match(input) {
    		format = input.clone();
    	}
    	else {
    		println!("Invalid format provided.");
    		std::process::exit(0);
    	}
    }
    for (i, x) in files.iter().enumerate() {
    	convert(command_str.clone(), x.to_string(), 
    		resize.to_string(), i+1, pretty, format.to_string());
    }
}

fn find_binary_windows() -> String {
	let find = Command::new("cmd")
        .args(&["/C", "where convert.exe"])
        .output()
        .expect("failed to execute process");
    let find = String::from_utf8(find.stdout).unwrap();
    let mut n = "";
    for i in find.split("\n") {
    	let mut n = i.to_string();
    	n.pop();
    	if n.contains("ImageMagick") {
	    	return n.to_string()
	    }
    }
    std::process::exit(0);
    //n.to_string()
}

fn find_binary_linux() -> String {
    let file = Path::new("/usr/bin/convert").exists();
    if !file {
        std::process::exit(0);
    }
    "/usr/bin/convert".to_string()
}

fn convert(command: String, file: String, resize: String, count: usize, pretty: bool, format: String) {
	let file_format = Regex::new(r"\.(?i)(jpg|jpeg|gif|png)$").unwrap();
	let mut new_file = file_format.replace(&file, "").to_string();
	new_file.push_str(&format);
	if &resize.chars().count() > &1 {
		if pretty == true {
			println!("({}): Resizing {} into {}", &count, &file, &resize);
			Command::new(&command)
				.args(&["-resize", &resize, &file, &file])
				.output()
				.expect("failed to execute process");
		}
		else {
			Command::new(&command)
				.args(&["-resize", &resize, &file, &file])
				.output()
				.expect("failed to execute process");
		}
	}
	if &format.chars().count() > &1 && &file != &new_file {
		if pretty == true {
			println!("({}): Converting {} into {}", &count, &file, &new_file);
			Command::new(&command)
				.args(&[&file, &new_file.to_string()])
				.output()
				.expect("failed to execute process");
		}
		else {
			Command::new(&command)
				.args(&[&file, &new_file.to_string()])
				.output()
				.expect("failed to execute process");
		}
		if &file != &new_file {
			fs::remove_file(&file);
		}
	}
}