use std::fs::OpenOptions;
use std::io::{Read, stdin, Write};

fn main() {
    let path = "C:\\Users\\12\\IdeaProjects\
    \\TrainingRust\\hello.txt";
    let mut f = OpenOptions::new()
        .write(true)
        .read(true)
        .create(true)
        .open(path)
        .expect("Failed to open file");
    let file_data =
        f.read_to_string(&mut String::new())
            .expect("Failed to read file");
    let mut input = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    f.write_all(input.as_bytes())
        .expect("Failed to write to file");
    println!("{}: ", file_data);
}
