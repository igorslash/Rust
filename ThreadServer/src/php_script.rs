use std::io::{BufRead, BufReader};
use std::process::Command;
use std::net::TcpStream;
use crate::{NOT_FOUND_HTML, STATUS_NOT_FOUND, STATUS_OK};

fn php_connection(stream: TcpStream) {
    let buf_read = BufReader::new(&stream);
    let requests = buf_read.lines().next().unwrap().unwrap();
    let (status_line, contents) = if requests
        .starts_with("GET /index.php HTTP/1.1") {
        let output = execute_php_script("index.php");
        (STATUS_OK, output)
    } else {
        (STATUS_NOT_FOUND, NOT_FOUND_HTML.to_string())
    };

    // ...
}
fn execute_php_script(filename: &str) -> String {
    let output = Command::new("cgi-fcgi")
        .arg("-bind")
        .arg("-connect")
        .arg("/var/run/php-fpm/php-fpm.sock")
        .arg("-query")
        .arg(filename)
        .output()
        .expect("Failed to execute PHP script");

    String::from_utf8_lossy(&output.stdout).to_string()
}
