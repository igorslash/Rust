use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let path = "hello.txt";
    std::fs::remove_file(path)
        .expect("Unable to remove file");
    std::rename("hello.txt", "hello23.txt")
        .expect("Unable to rename file");
    let mut f = File::create(path)
        .expect("Unable to create file");
    f.write_all(b"Hello, world!".as_ref())
        .expect("Unable to write to file");
    //b is a byte slice
    let mut f_open = File::open(path)
        .expect("Unable to open file");
    let mut s = String::new();
    f_open.read_to_string(&mut s)
        .expect("Unable to read file");
    println!("{}", s);

}