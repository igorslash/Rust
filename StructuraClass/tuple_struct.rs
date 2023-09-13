#[derive(Debug)]
struct Str(i32, String, i64);

fn main() {
    let object = Str(1, "Hello".to_string(), 2);
    println!("{:?}", object);
}