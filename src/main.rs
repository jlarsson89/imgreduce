extern crate execute;
use std::process::Command;
use execute::Execute;
use which::which;
//use std::path::PathBuf;
use std::env;

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
}
