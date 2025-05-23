use clap::{Arg, Command};
use std::process::Command;

pub fn parse_cli() -> Result<Command, String> {
    Ok(Command::new("Console List")
           .arg(
               Arg::new("command")
                   .required(true)
                   .possible_values(&["add", "list", "delete", "exit"])
                   .index(1),
           )
           .arg(Arg::new("content").help("Содержимое заметки"))
           .arg(Arg::new("id").help("ID заметки для удаления")))
}