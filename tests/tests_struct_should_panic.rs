fn main()  {

}
pub struct Guess {
    value: i32,
}
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }else if value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }
        Guess { value }
    }
}
#[cfg(test)]
mod tests {
    use crate::Guess;
    #[test]
    #[should_panic(expected = "Guess value must be between 1 and 100, got 200.")]
    fn greater_than_100() {
        Guess::new(200);
    }
}