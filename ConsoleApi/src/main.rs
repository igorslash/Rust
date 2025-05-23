#![warn(clippy::all, clippy::pedantic)]

mod cli;
mod note;
mod utils;

fn main() {
    let args = parse_cli().expect("Ошибка парсинга аргументов");
    let command = args.get_one::<String>("command").unwrap();
    let notes_path = "notes.json";

    match command.as_str() {
        "add" => {
            let content = args.get_one::<String>("content").cloned().unwrap_or_default();
            if !content.is_empty() {
                add_note(notes_path, &content);
            } else {
                println!("Ошибка: пустое содержимое заметки");
            }
        }
        "list" => list_notes(notes_path),
        "delete" => {
            let id = args.get_one::<String>("id").and_then(|s| s.parse().ok());
            if let Some(id) = id {
                delete_note(notes_path, id);
            } else {
                println!("Ошибка: неверный ID");
            }
        }
        _ => {}
    }
}

fn add_note(path: &str, content: &str) {
    let notes = read_notes_from_file(path).unwrap_or_default();
    let new_id = if !notes.is_empty() { notes.last().unwrap().id + 1 } else { 1 };
    let note = Note {
        id: new_id,
        content: content.to_string(),
        created_at: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Ошибка получения времени")
            .as_secs()
            .to_string(),
    };
    write_notes_to_file(path, &[note]).unwrap();
    println!("Заметка добавлена (ID: {})", new_id);
}

fn list_notes(path: &str) {
    let notes = read_notes_from_file(path).unwrap_or_default();
    if notes.is_empty() {
        println!("Нет заметок");
    } else {
        for note in &notes {
            println!(
                "ID: {}\nДата: {}\nСодержимое: {}\n",
                note.id, note.created_at, note.content
            );
        }
    }
}

fn delete_note(path: &str, id: u64) {
    let mut notes = read_notes_from_file(path).unwrap_or_default();
    notes.retain(|note| note.id != id);
    write_notes_to_file(path, &notes).unwrap();
    println!("Заметка с ID {} удалена", id);
}
