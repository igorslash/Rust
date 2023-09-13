fn main () {
    let mut  string = String::from("Hello");
    let string2 = &string;//& так можно без нет
    let string3 = &string;
    println!("{}, {}", string2, string3);
//----------------------------------------------------------------------
    let string = ret();
    println!("{}", string);

}
fn ret() -> &String {
    let string = String::from("Hello");//error c &
    &string
}