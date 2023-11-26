fn add_num(a: i32, b: i32) -> i32 {
    a + b
}
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}
#[cfg(test)]
mod tests {
    #[test]
    #[ignore]// Ignore this test
    //проигнорированные тесты, вы можете
    // воспользоваться командой cargo test -- --ignored:
    fn example() {
        assert_eq!(2 + 2, 4);
    }
}