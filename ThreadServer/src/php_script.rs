use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::process::Command;
use crate::{NOT_FOUND_HTML, STATUS_NOT_FOUND, STATUS_OK};

fn php_connection(stream: TcpStream) {
    /// Create a buffered reader for the TCP stream
    let buf_read = BufReader::new(&stream);

    /// Read the first line from the buffered reader
    let requests = buf_read.lines().next().unwrap().unwrap();

    /// Check if the request is for "GET /index.php HTTP/1.1"
    let (status_line, contents) = if requests.starts_with
    ("GET /index.php HTTP/1.1") {
        // Execute the PHP script "index.php"
        let output = execute_php_script("index.php");
        (STATUS_OK, output)
    } else {
        /// Request is not for "GET /index.php HTTP/1.1", return a
        /// not found status and HTML
        (STATUS_NOT_FOUND, NOT_FOUND_HTML.to_string())
    };

    // ...
}

/// Function to execute a PHP script
fn execute_php_script(filename: &str) -> String {
    // Execute the PHP script using the "cgi-fcgi" command
    let output = Command::new("cgi-fcgi")
        .arg("-bind")
        .arg("-connect")
        .arg("/var/run/php-fpm/php-fpm.sock")
        .arg("-query")
        .arg(filename)
        .output()
        .expect("Failed to execute PHP script");

    /// Convert the output to a string and return it
    String::from_utf8_lossy(&output.stdout).to_string()
}
