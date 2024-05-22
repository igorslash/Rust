#![warn(clippy::all, clippy::pedantic)]
use core::cmp::min;

fn main() {
    println!("{}", repetition!(1u32));
    println!("{}", repetition!(1u32 + 2, 2u32));
    println!("{}", repetition!(1u32 * 2, 2u32, 1u32));
}
#[macro_export]
macro_rules! repetition {
    //повторение макроса
    ($x:expr) => ($x);
    ($x:expr, $($y:expr),+) => {
        min($x, repetition!($($y),+))// + повторение 1 раз
        //min($x, repetition!($($y),*))// * повторение больше раз
    };
}