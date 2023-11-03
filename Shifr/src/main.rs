extern crate core;
use std::fs::File;
use std::io::{Read, Write};
use std::num::IntErrorKind;

fn main() {
    loop {
        let input = get_input("What would you like to do?");
        let input_file = match File::open(&input) {
            Ok(file) => file,
            Err(err) => {
                println!("Could not open file: {err}");
                continue;
            }
        };
        let key = match  get_input("Enter key encryption/decryption")
            .parse::<u8>() {
            Ok(key) => key,
            Err(err) => {
                match err.kind() {
                    IntErrorKind::Empty => println!("Key must not be empty"),
                    IntErrorKind::InvalidDigit => println!
                    ("Key must contain only digits"),
                    IntErrorKind::PosOverflow => println!("Key too range [0,255]"),
                    _ => println!("Could not parse key: {err}"),
                }
                continue;
            }
        };
        let mut reader = std::io::BufReader::new(input_file);
        let mut input_data = Vec::new();
        if let Err(err) = reader.read_to_end(&mut input_data) {
            println!("Could not read file data: {err}");
            continue;
        }
        let processed_data = process_file_data(input_data, key);
        let output_file = get_input("Enter output file input");
        let mut output_file = std::io::BufWriter::new
            (File::create(output_file).unwrap());
        if let Err(err) = output_file.write_all(&processed_data) {
            println!("Error writing to output file: {err}");
            continue;
        }
    };
        println!();
}
fn get_input(query: &str) -> String {
    print!("{query}:");
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_owned()
}
fn process_file_data(data: Vec<u8>, key: u8) -> Vec<u8> {
    let mut processed_data = Vec::with_capacity(data.len());
    for byte in data {
        processed_data.push(byte ^ key);
    }
    processed_data
}

