//В Rust макросы могут принимать аргументы различных типов,
// таких как идентификаторы (ident), выражения (expr),
// блоки кода (block), типы (ty), образцы (pat) и другие.
// Кроме того, есть специальные обобщенные типы для аргументов
// макросов, такие как $($var:ty)* для захвата нескольких типов.


fn main() {
    let post = Post::new(1, String::from("Hello"));
    println!("{}", get_title!(post));
}
struct Post {
    id: i32,
    title: String,

}
impl Post {
    fn new(id: i32, title: String) -> Post {
        Post { id, title }
    }
    fn get_id(&self) -> i32 {
        self.id
    }
}
#[macro_export]
macro_rules! get_title {
    ($post:ident) => {
        match $post {
            ref post => post.title.clone(),
            _ => String::from("no title"),
        }
    };
}
#[cfg(test)]
fn test() {
    let post = Post::new(String::from("Hello"));
    assert_eq!(get_title!(post), "Hello");
    #[test]
    fn test_get_id() {
        let post = Post::new(1);
        assert_eq!(post.get_id(), 1);
    }
}