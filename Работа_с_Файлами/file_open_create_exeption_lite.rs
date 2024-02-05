use std::fs::File;
use std::io::{Read, Write};

fn main() {
    open_read_write_file().expect("Could not open file");
}
fn open_read_write_file() -> Result<String, std::io::Error> {
    let mut writer = File::create("Hello.txt")
        .expect("Could not create file");
    let mut file = String::new();
    File::open("Hello.txt")?.read_to_string(&mut file)?;
    writer.write_all("Hello, from Rust".as_bytes())?;
    Ok(file)
}