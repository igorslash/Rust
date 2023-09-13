fn main() {
    let str = String::from("Hello Igor");
    let slice = &str[1..5];
    println!("{}", slice);
}