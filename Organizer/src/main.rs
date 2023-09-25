use std::io::Write;
use std::path::PathBuf;

fn get_input(query: &str) -> std::io::Result<String> {
    println!("{}", query);
    std::io::stdout().flush()?;

    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer)?;
    Ok(buffer.trim().to_owned())
}
fn organize_dir(dir_path: PathBuf) {
    if !dir_path.exists() {
        println!("{} does not exist", dir_path.display());
    }else {
       let files = std::fs::read_dir(dir_path).unwrap();
        println!("{} has {} files", dir_path.display(), files.count());
    }
    for file in dir_path.read_dir().unwrap() {
        let file = file.unwrap();
        if file.path().is_dir() {
            organize_dir(file.path());
            continue
        }
        let file_extetion = match file.path().extension() {
            None => {
                println!("{} has no extension", file.path().display());
                continue
            },
            Some(extension) => match extension.to_str() {
                None => continue,
                Some(extension) => extension.to_lowercase()

            }
        };
        let extetion_dir = PathBuf::from(dir_path
            .join(file_extetion));
        create_dir(extetion_dir);
        move_file(file.path(),
                  extetion_dir.join(file.file_name().unwrap()));
    }
}
fn create_dir(dir_path: PathBuf) {
    if !dir_path.exists() {
        std::fs::create_dir(dir_path).unwrap();
    }
}
fn move_file(from: PathBuf, to: PathBuf) {
    std::fs::rename(from, to).unwrap();
}

fn main() {
    loop {
        let dir_path = match get_input("Enter the path") {
            Ok(dir_path) => dir_path,
            Err(err) => {
                println!("Invalid path {}", err);
                continue;
            }
        };
        let now = std::time::Instant::now();
        organize_dir(PathBuf::from(dir_path));
        println!("Done in {}ms", now.elapsed().as_millis());
    }
}


