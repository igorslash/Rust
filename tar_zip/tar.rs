use std::fs::File;
use std::io::prelude::*;
use tar::Builder;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Создаем новый архив TAR
    let file = File::create("archive.tar")
        .expect_err("Could not create file");
    let mut builder = Builder::new(file);

    // Добавляем файлы в архив
    let mut file1 = File::open("hello.txt")?;
    builder.append_file("file1.txt", &mut file1)?;

    // Завершаем архивирование
    builder.finish()?;

    Ok(())
}