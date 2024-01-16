use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::thread::spawn;
use std::time::Duration;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Port Scanner")
        .version("1.0")
        .author("Your Name")
        .about("A command-line tool to scan ports on a given IP address")
        .arg(
            Arg::with_name("IP_ADDRESS")
                .help("Sets the IP address to scan")
                .required(true)
                .index(1),
        )
        .get_matches();

    let ip_address = matches
        .value_of("IP_ADDRESS")
        .expect("IP_ADDRESS is required");

    let ip = IpAddr::from_str(ip_address).expect("Invalid IP address");

    let threads = spawn(move || {
        for port in 1..=65535 {
            let target = format!("{}:{}", ip, port);
            thread::sleep(Duration::from_millis(1000));
            match TcpStream::connect_timeout(&target.parse().unwrap(),
                                             Duration::from_secs(1)) {
                Ok(_) => println!("Port {} is open", port),
                Err(_) => continue,
            }
        }
    });
    threads.join().unwrap();
}
//clap = "2.33"