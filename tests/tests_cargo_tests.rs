pub fn add_plus(x: i32) -> i32 {
    x + 2
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]//вызываем тест по отдельности cargo test add_two, cargo test add
    //запустит фильтр из нескольких тестов начинающихся с add
    fn add_two() {
        assert_eq!(4, add_plus(2));
    }
    #[test]
    fn add_three() {
        assert_eq!(5, add_plus(3));
    }
}