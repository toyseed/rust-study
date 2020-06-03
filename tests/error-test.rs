use std::{fs, fs::{File}};
use std::io::Read;
use std::io;

fn propagate_error() -> Result<String, io::Error> {
    let mut f = File::open("not_exist_file.txt")?;
    let mut s = String::new();

    f.read_to_string(&mut s)?;

    Ok(s)
}

fn propagate_error_shortly() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("no_file.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn same_but_shorter() -> Result<String, io::Error> {
    fs::read_to_string("no_file.txt")
}

#[test]
fn test_propagate_error() {
    propagate_error();

    same_but_shorter();
}
