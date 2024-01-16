fn main() {
    let data = DataBase::ID { id: 1 };
    match data {
        DataBase::ID { id : id_variable@1..=10,
        } => println!("{}", id_variable),

        DataBase::ID {id: 10..=15} => println!("id is not"),

        DataBase::ID {id} => println!("{}", id)
    }
}
enum DataBase {
    ID{id: i32},
}