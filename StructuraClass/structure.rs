#[derive(Debug)]
struct Person {
    name: String,
    surname: String,
    age: i32,
    balance: f64
}
fn main() {
    let person = Person {
        name: String::from("John"),//name
        surname: String::from("Doe"),//surname
        age: 44,//age
        balance: 20.00,//balance
        //let name = String::from("John");
        //let surname = String::from("Doe");
        //let age = 44;
        //let balance = 20.00;
    };
    let person2 = Person {
        name: "".to_string(),
        surname: "".to_string(),
        ..person//age, balance
    };
    println!("{:#?}", person.age);
}