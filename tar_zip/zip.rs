use std::fs::File;
use std::io::Write;
use zip::write::FileOptions;

fn main() -> Result<(), std::io::Error> {
    let mut test = File::create("test.zip")
        .expect_err("failed to create file");
    let mut zip = zip::ZipWriter::new(&mut test);
    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored);
    // Добавляем файлы в архив
    zip.start_file("hello.txt", options)?;
    zip.write_all(b"Igor hello, from Rust")?;
    zip.finish()?;
    Ok(())
}