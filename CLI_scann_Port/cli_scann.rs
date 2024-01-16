use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::thread::spawn;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let parsed_args = Args::new(&args);
    let threads = spawn(move || {
        for port in 1..=65535 {
            let address = format!("{}:{}", parsed_args.ip_address, port);
            std::thread::sleep(std::time::Duration::from_millis(1000));
            match TcpStream::connect_timeout(&address.parse().unwrap(),
                                             std::time::Duration::from_secs(1)) {
                Ok(_) => println!("{} is open", address),
                Err(_) => continue,
            }
        }

    });
    threads.join();
}
struct Args {
    flags: Vec<String>,
    ip_address: IpAddr,
    threads: u16,
}

impl Args {
    fn new(args: &[String]) -> Args {
        if args.len() != 2 && args.len() > 4 {
            writeln!(io::stderr(), "Usage: {} <IP_ADDRESS>", args[0]).unwrap();
            std::process::exit(1);
        }

        let ip_address = IpAddr::from_str(&args[1])
            .expect("invalid IP address");

        Args {
            flags: args[1..].to_vec(),
            ip_address,
            threads: 4,
        }
    }
}