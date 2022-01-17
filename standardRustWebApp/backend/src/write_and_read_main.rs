use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Write file
    let mut create_file = File::create("test.txt")?;
    create_file.write_all(b"Det her er en test :D for satan")?;
    //
    // Read file and print contents
    let mut open_file = File::open("test.txt")?;
    let mut contents = String::new();
    open_file.read_to_string(&mut contents)?;
    assert_eq!(contents, "Det her er en test :D for satan");
    println!("{contents}");
    Ok(())
    //
}
