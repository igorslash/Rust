#![warn(clippy::all, clippy::pedantic)]
fn main() {
    calculate!(mul 1 * 2);
}
#[macro_export]
macro_rules! calculate {
    (mul $e:expr) => {
        {
            let val = $e;
            println!("{} = {}", stringify!($e), val);
        }
    };
}