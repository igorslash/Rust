use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: u64,
    pub created_at: String,
    pub content: String,
}
pub fn read_notes_from_file(file: &str) -> Result<Vec<Note>, String> {
    let file = File::open(file).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);
    let notes: Vec<Note> = serde_json::from_reader(reader).map_err(|e| e.to_string());
    Ok(notes)
}
pub fn write_notes_to_file(file: &str, notes: &Vec<Note>) -> Result<(), String> {
    let file = OpenOptions::new()
    .write(true)
    .truncate(true)
    .create(true)
    .open(file)
    .map_err(|e| e.to_string())?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, notes).map_err(|e| e.to_string())

}