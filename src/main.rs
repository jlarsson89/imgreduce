extern crate regex;
extern crate bytesize;
use std::process::Command;
use std::path::Path;
use std::{env, fs, io};
use std::sync::Mutex;
use clap::{App, Arg};
use regex::Regex;
use bytesize::ByteSize;

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
pub struct Files {
	path: String,
	name: String,
	size: i64,
}

impl Files {
	fn update_path(&mut self, new_path: String) {
		self.path = new_path;
	}
	fn update_name(&mut self, new_name: String) {
		self.name = new_name;
	}
	fn update_size(&mut self, new_size: i64) {
		self.size = new_size;
	}
}

lazy_static! {
	static ref FILES: Mutex<Vec<Files>> = Mutex::new(vec![]);
}

fn main() {
	let mut command_str = "".to_string();
	let mut resize = "";
	let mut pretty = false;
	let mut format = "";
	let mut base = ".";
	let mut recursion = false;
	let file_format = Regex::new(r"^.*\.(?i)(jpg|jpeg|gif|png)$").unwrap();
	let matches = App::new("imgreduce")
		.arg(
			Arg::with_name("dir")
			.help("Directory")
			.takes_value(true)
			.short("d")
			)
		.arg(
			Arg::with_name("resize")
			.help("Resolution to resize to")
			.takes_value(true)
			.short("s")
			)
		.arg(
			Arg::with_name("format")
			.help("Convert into a new file format")
			.takes_value(true)
			.possible_values(&[".jpg", ".jpeg", ".gif", ".png"])
			.short("f")
			)
		.arg(
			Arg::with_name("recursion")
			.help("Enable recursion")
			.takes_value(false)
			.short("r")
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
    	base = matches.value_of("dir").unwrap();
    }
    if matches.is_present("pretty") {
    	pretty = true;
    }
    if matches.is_present("recursion") {
    	recursion = true;
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
    add_files(base.to_string(), recursion);
    let mut f = FILES.lock().unwrap();
    if f.len() > 0 {
    	let mut start_size = 0;
    	for i in f.iter() {
    		start_size += i.size;
    	}
    	if format != "" {
    		for i in f.iter_mut() {
    			convert_files(i, format.to_string(), command_str.clone(), pretty);
    		}
    	}
    	if resize != "" {
    		for i in f.iter_mut() {
    			resize_files(i, resize.to_string(), command_str.clone(), pretty);
    		}
    	}
    	let mut new_size = 0;
    	for i in f.iter() {
    		new_size += i.size;
    	}
    	if pretty {
    		println!("A total of {} images have been found.", f.len());
	    	if start_size != new_size {
	    		print!("Before modification, they used a total of {}, after modification ", ByteSize(start_size as u64));
	    		if start_size > new_size {
	    			print!("they use {}, ", ByteSize(new_size as u64));
	    			let diff = start_size - new_size;
	    			print!("saving a total of {}.\n", ByteSize(diff as u64));
	    		}
	    		else {
	    			print!("they use {}, ", ByteSize(new_size as u64));
	    			let diff = new_size - start_size;
	    			print!("using {} more disk space.\n", ByteSize(diff as u64));
	    		}
	    	}
	    }
	}
}

fn find_binary_windows() -> String {
	let find = Command::new("cmd")
        .args(&["/C", "where convert.exe"])
        .output()
        .expect("failed to execute process");
    let find = String::from_utf8(find.stdout).unwrap();
    for i in find.split("\n") {
    	let mut n = i.to_string();
    	n.pop();
    	if n.contains("ImageMagick") {
	    	return n.to_string()
	    }
    }
    std::process::exit(0);
}

fn find_binary_linux() -> String {
    let file = Path::new("/usr/bin/convert").exists();
    if !file {
        std::process::exit(0);
    }
    "/usr/bin/convert".to_string()
}

fn add_files(base: String, recursion: bool) {
	if Path::new(&base).exists() {
		for entry_res in fs::read_dir(&base).unwrap() {
			let entry = entry_res.unwrap();
			let file_name_buf = entry.file_name();
			let file_name = file_name_buf.to_str().unwrap();
			if entry.file_type().unwrap().is_file() && !file_name.starts_with(".") {
    			let re = Regex::new(r"\.(?i)(jpg|jpeg|gif|png)$").unwrap();
    			let metadata = fs::metadata(entry.path());
    			let s: i64 = metadata.unwrap().len() as i64;
    			let path = entry.path();
    			let p = path.clone().into_os_string().into_string().unwrap();
				if re.is_match(&p) {
					let f: Files = Files{path: p, name: file_name.to_string(), size: s};
					FILES.lock().unwrap().push(f);
				}
			}
			if recursion && entry.file_type().unwrap().is_dir() && !file_name.starts_with(".") {
				let mut dir = entry.path().display().to_string();
				dir.push('/');
				if !is_folder_empty(&dir).unwrap() {
					add_files(dir, recursion);
				}
			}
		}
	}
}

fn convert_files(i: &mut Files, format: String, command: String, pretty: bool) {
	let start_path = &i.path;
	let start_name = &i.name;
	let file_format = Regex::new(r"\.(?i)(jpg|jpeg|gif|png)$").unwrap();
	let mut new_file = file_format.replace(&start_name, "").to_string();
	let mut new_path = file_format.replace(&start_path, "").to_string();
	new_file.push_str(&format);
	new_path.push_str(&format);
	if start_path.eq(&new_path) && start_name.eq(&new_file) {

	}
	else {
		if pretty {
			println!("Converting {} into {}.", start_name, new_file);
		}
		Command::new(&command)
			.args(&[&start_path, &new_path.to_string()])
			.output()
			.expect("failed to execute process");
		fs::remove_file(&start_path);
		let metadata = fs::metadata(&new_path);
		let s: i64 = metadata.unwrap().len() as i64;
		i.update_path(new_path);
		i.update_name(new_file);
		i.update_size(s);
	}
}

fn resize_files(i: &mut Files, new_resolution: String, command: String, pretty: bool) {
	let start_path = &i.path;
	let start_name = &i.name;
	if pretty {
		println!("Resizing {} to {}.", start_name, new_resolution);
	}
	Command::new(&command)
			.args(&["-resize", &new_resolution, &start_path, &start_path])
			.output()
			.expect("failed to execute process");
	let metadata = fs::metadata(&start_path);
	let s: i64 = metadata.unwrap().len() as i64;
	i.update_size(s);
}

fn is_folder_empty(path: impl AsRef<Path>) -> io::Result<bool> {
	Ok(fs::read_dir(path)?.take(1).count() == 0)
}