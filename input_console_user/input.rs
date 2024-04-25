#![warn(clippy::all, clippy::pedantic)]

use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let username = input_user_console("Enter username: ");
    let password = input_user_console("Enter password: ");
    let hashed_password = hash_password(&password);
    println!("Username: {}, Password: {}", username, hashed_password);
}
fn input_user_console(prompt: &str) ->  String {
    let mut input = String::new();
    println!("{}", prompt);
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()

}
fn hash_password(password: &str) -> String {
    let hashed_password = hash(password, DEFAULT_COST)
        .expect("Failed to hash password");
    hashed_password
}