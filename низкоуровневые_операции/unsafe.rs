fn main() {
    let mut number = 3;
    let num = &number as *const i32;
    let num2 = &mut number as *mut i32;
    unsafe {
        println!("{} {}", *num, *num2);
    }
}