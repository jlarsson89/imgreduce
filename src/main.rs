extern crate execute;
use std::process::Command;
use execute::Execute;
use which::which;
use std::path::PathBuf;

fn main() {
    let result = which::which("convert").unwrap();
    assert_eq!(result, PathBuf::from("/usr/bin/convert"));
    let mut command = Command::new("convert");
    command.execute_output().unwrap();
    //println!("Hello, world!");
}
