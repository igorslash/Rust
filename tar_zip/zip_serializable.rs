use std::fs::File;
use std::io::prelude::*;
use zip::ZipArchive;
use serde_json;

fn main() {
    // Создание файла и запись в него
    let mut file = File::create("C:\\Users\\12\\IdeaProjects\
    \\TrainingRust\\src\\Работа_с_Файлами\\file.txt")?;
    file.write_all(b"Hello, world!")?;
    let serialized = serde_json::to_string(&file)?;
    file.write_all(serialized.as_bytes())?;

    // Извлечение файла из zip-архива
    let mut archive = zip::ZipArchive::new(File::open("path/to/your\
    /archive.zip")?)?;
    archive.extract("file.txt", "C:\\Users\\12\\IdeaProjects\
    \\TrainingRust\\src\\Работа_с_Файлами\\file.txt")?;
    let deserialized = serde_json::from_str(&archive.read("file.txt")?)?;
    assert_eq!(deserialized, file);
    Ok(()).expect("Failed to write file");
}