#![warn(clippy::all, clippy::pedantic)]
//перегрузка macros
#[macro_export]
macro_rules! example {
    ($left:expr, $right:expr) => {
        $left + $right
    };
    ($left:expr, $right:expr,) => {
        $left * $right
    }
}

fn main() {
    let result = example!(1, 2);
    println!("{}", result);

    let result1 = example!("hello ".to_owned(), "igor");
    println!("{}", result1);

}