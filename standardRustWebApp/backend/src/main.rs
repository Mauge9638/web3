use chrono::*;
use std::fs::File;
use std::io::prelude::*;

fn get_time() {
    let utc: DateTime<Local> = Local::now();
    println!("{utc}");
}

fn write_to_file(filename: &str, input: &'static [u8]) -> std::io::Result<()> {
    let mut create_file = File::create(filename)?;
    create_file.write_all(input)?;
    Ok(())
}

fn open_and_read_file(filename: &str) -> std::io::Result<()> {
    let mut open_file = File::open(filename)?;
    let mut contents = String::new();
    open_file.read_to_string(&mut contents)?;
    println!("{contents}");
    Ok(())
}

fn main() {
    // Write file
    /* let mut create_file = File::create("test.txt")?;
    create_file.write_all(b"Det her er en test :D for satan")?; */
    //
    // Read file and print contents
    //let filename = get_time();
    match write_to_file(filename, b"Det her er en test :D for satan jo") {
        Ok(..) => println!("File created"),
        Err(..) => println!("Something went wrong"),
    };
    match open_and_read_file(filename) {
        Ok(..) => println!("File read"),
        Err(..) => println!("Something went wrong"),
    };
    //
}
