use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Note {
    id: u64,
    content: String,
    created_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_serialize() {
        let note = Note {
            id: 1,
            content: "Привет, мир!".to_string(),
            created_at: "1694000000".to_string(),
        };

        let json = serde_json::to_string(&note).unwrap();
        assert_eq!(json, "{\"id\":1,\"content\":\"Привет, мир!\",\"created_at\":\"1694000000\"}");
    }

    #[test]
    fn test_note_deserialize() {
        let json = r#"{"id":2,"content":"Test","created_at":"1694000001"}"#;
        let note: Note = serde_json::from_str(json).unwrap();

        assert_eq!(note.id, 2);
        assert_eq!(note.content, "Test");
        assert_eq!(note.created_at, "1694000001");
    }
}
