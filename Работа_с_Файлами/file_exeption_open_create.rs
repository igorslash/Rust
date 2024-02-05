use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Read, Write};

fn main()  {
    File::open("hello.txt").unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", e);
        }
    });
    let path = "C:\\Users\\12\\IdeaProjects\\TrainingRust\\hello.txt";
    let mut file_open = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path).expect("failed to open file");
    let mut str = String::new();
    file_open.write_all("Igor, hello from Rust!".as_bytes())
        .expect_err("failed to write");
    file_open.read_to_string(&mut str).unwrap();
    println!("{}", str);
}