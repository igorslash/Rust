fn main() {
    let s = String::from("hello");
    let str = String::from("hel");
    check_str!(s, str);

    let vec = vec![1, 2, 3];
    let x = 2;
    comparison_value!(vec, x);
}
#[macro_export]
macro_rules! check_str {
    ($s:expr, $str:expr) => {
        if $s.ends_with("o") {
            println!("Hello, {}!", $s);
        }else {
            println!("Hello, {}!", $str);
        }
    };
}
//vec![1, 2, 3]
#[macro_export]
macro_rules! comparison_value {
    ($vec:pat, $x:expr) => {
        match $x {
            $vec => {
                println!("equal");
            }
            _ => println!("not equal"),
        }
    };
}