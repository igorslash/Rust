fn main() {
    let email = "igor@gmail.com";
    let key = 34;
    println!("{}", encrypt(&text, key));

}
fn encrypt(text: &str, key: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push((c as u8 + key) as char);
    }
    result
}
fn decrypt(text: &str, key: u8) -> String {
    let mut result = String::new();
    for c in text.chars() {
        result.push((c as u8 - key) as char);
    }
    result
}