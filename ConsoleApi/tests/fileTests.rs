use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};

fn add_note(notes: &mut Vec<Note>, content: &str) {
    let id = if !notes.is_empty() { notes.last().unwrap().id + 1 } else { 1 };
    let created_at = "1694000000".to_string(); // Простая статичная временная метка
    notes.push(Note {
        id,
        content: content.to_string(),
        created_at,
    });
}

fn delete_note_by_id(notes: &mut Vec<Note>, target_id: u64) -> bool {
    let index = notes.iter().position(|n| n.id == target_id);
    if let Some(i) = index {
        notes.remove(i);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_note() {
        let mut notes = Vec::new();
        add_note(&mut notes, "Первая заметка");
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].id, 1);
        assert_eq!(notes[0].content, "Первая заметка");

        add_note(&mut notes, "Вторая заметка");
        assert_eq!(notes.len(), 2);
        assert_eq!(notes[1].id, 2);
    }

    #[test]
    fn test_delete_note_by_id() {
        let mut notes = vec![
            Note { id: 1, content: "Заметка 1".to_string(), created_at: "time1".to_string() },
            Note { id: 2, content: "Заметка 2".to_string(), created_at: "time2".to_string() },
        ];

        assert!(delete_note_by_id(&mut notes, 2));
        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].id, 1);

        // Попытка удалить несуществующий ID
        assert!(!delete_note_by_id(&mut notes, 3));
        assert_eq!(notes.len(), 1);
    }
}
