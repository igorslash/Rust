use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::Path;

fn save_notes(notes: &[Note], path: &str) -> Result<(), std::io::Error> {
    let file = File::create(path)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, notes)?;
    Ok(())
}

fn load_notes(path: &str) -> Result<Vec<Note>, std::io::Error> {
    if !Path::new(path).exists() {
        return Ok(Vec::new());
    }
    let file = File::open(path)?;
    let reader = std::io::BufReader::new(file);
    serde_json::from_reader(reader)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_and_load_notes() {
        // Путь к временному файлу для теста
        let path = "tests/test_notes.json";

        let note1 = Note {
            id: 1,
            content: "Заметка 1".to_string(),
            created_at: "1694000000".to_string(),
        };
        let note2 = Note {
            id: 2,
            content: "Заметка 2".to_string(),
            created_at: "1694000001".to_string(),
        };

        // Сохраняем заметки
        save_notes(&[note1, note2], path).unwrap();

        // Загружаем и проверяем
        let loaded = load_notes(path).unwrap();
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, 1);
        assert_eq!(loaded[1].id, 2);

        // Очищаем файл после теста
        std::fs::remove_file(path).unwrap_or_default();
    }

    #[test]
    fn test_load_empty_notes() {
        let path = "tests/empty_notes.json";
        let notes = load_notes(path).unwrap();

        assert!(notes.is_empty());
    }
}
