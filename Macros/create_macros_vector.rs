fn main() {
    create_vector!(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, "hello", "world");
}
#[macro_export]
macro_rules! create_vector {
    ($($x:expr),*) => {
        {
            std::thread::spawn(move || {
                $(
                    let mut v = Vec::new();
                    std::thread::sleep(std::time::Duration::from_secs(1));
                    v.push($x);
                )*
            });
        }
    };
}
/* This code represents a macro in Rust that creates a vector and performs
an append operation on multiple threads.
The create_vector macro takes several expressions and creates
a new thread for each of them, which adds an element to the vector.
 */