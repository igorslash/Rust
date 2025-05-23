#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_notes_not_exists() {
        let path = "nonexistent_file.json";
        assert!(load_notes(path).is_ok());
    }

    #[test]
    fn test_save_notes_error() {
        // Пытаемся сохранить в недоступное место
        let notes = vec![Note { id: 1, content: "Test".to_string(), created_at: "time".to_string() }];
        assert!(save_notes(&notes, "/nonexistent/path/test.json").is_err());
    }
}
