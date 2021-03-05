use std::io;
use std::fs;
use std::fs::File;
use std::error::Error;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("Cargo.toml")
}

fn read_file() -> Result<(), Box<dyn Error>> {
    let _ = File::open("Cargo.toml")?;
    Ok(())
}

#[test]
fn file_read_works() {
    println!("{}", read_username_from_file().unwrap());
    let r=read_file();
    assert!(r.is_ok());
}

