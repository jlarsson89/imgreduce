extern crate execute;
use std::process::Command;
use execute::Execute;
use which::which;
//use std::path::PathBuf;
use std::{env, fs, io};
use clap::{App, Arg};

fn main() {
    let result = which::which("convert").unwrap();
    //assert_eq!(result, PathBuf::from("/usr/bin/convert"));
    let mut command = Command::new("convert");
    command.execute_output().unwrap();
    //println!("Hello, world!");
    //println!("{}", env::consts::OS);
    let os = env::consts::OS;
    println!("{}", os);
    match os {
    	_ if os == "windows" => println!("1"),
    	_ if os == "linux" => println!("2"),
    	_ => println!("0"),
    }
    let current_dir = env::current_dir();
    println!(
        "Entries modified in the last 24 hours in {:?}:",
        current_dir
    );
    /*for entry in fs::read_dir(".").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            println!("{:?} is a dir", path);
        } else {
            println!("{:?} is a file", path);
        }
    }*/
    /*for entry in fs::read_dir(provided_dir).unwrap() {
    	let entry = entry.unwrap();
    	let path = entry.path();
    	println!("{:?}", path);
    }*/
}