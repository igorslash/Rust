use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let user = User { name: String::from("user"),
        password: String::from("password")};
    let token = String::from("ssecret");
    let password = "245ufo";
    let cost_hash = 12; // Увеличенная стоимость хэширования
    let hashed_password = bcrypt::hash(password, cost_hash).unwrap();
    authenticate!(token: token, user: user, password: password);
}
#[derive(Debug)]
struct User<T> {
    name: String,
    password: T,
}
#[macro_export]
macro_rules! authenticate {
    (token: $token:ident, user: $user:ident, password: $password:ident) => {
        match ($token == "ssecret") && ($user.name == "user")
        && ($password == "password") {
            true => {
                println!("Authentication successful! {}", stringify!($user));
            }
            false => {
                println!("Authentication failed! {}", stringify!($user));
            }
        }
    };
}