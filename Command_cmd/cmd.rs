use std::process::Command;

fn main() {
    let result = Command::new("chmod")
        .arg(&["+r", "-w", "/path/to/file"])
        .output();
    match result {
        Ok(output) =>
            if output.status.success() {
                println!("{}", String::from_utf8_lossy(&output.stdout));
            }else {
                let err = String::from_utf8_lossy(&output.stderr);
                println!("Ошибка при ограничении прав доступа к файлам: {}", err);
            },
        Err(error) => {
            println!("Ошибка выполнения команды 'chmod': {}", error);
        }
    }
    let result = Command::new("sudo")
        .arg(&["ls"])
        .output();
    match result {
        Ok(output) => {
            if output.status.success() {
                let output_lines =
                    String::from_utf8_lossy(&output.stdout);
                println!("Результат команды 'ls': {}", output_lines);
            }
        }
        Err(error) => {
            println!("Ошибка выполнения команды 'sudo': {}", error);
        }
    }
}