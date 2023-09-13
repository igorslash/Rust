fn main() {
    let x = five();

    let y = plus_one(4);
    println!({} {}, x, y);
}
fn five() -> i32 {
    5
}
fn plus_one(x: i32) -> i32 {
    x + 1
}