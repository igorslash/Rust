use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use threadpool::ThreadPool;

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
    //write ip address
    let ip_address = matches
        .value_of("IP_ADDRESS")
        .expect("IP_ADDRESS is required");

    let ip = IpAddr::from_str(ip_address)
        .expect("Invalid IP address");

    let pool = ThreadPool::new(num_cpus::get()); // create threadpool

    for port in 1..=65535 {
        let target = format!("{}:{}", ip, port);
        let pool = Arc::clone(&pool); // We create an arch for
        // use in closure

        pool.execute(move || {
            thread::sleep(Duration::from_millis(1000));
            match TcpStream::connect_timeout(&target.parse().unwrap(),
                                             Duration::from_secs(1)) {
                Ok(_) => println!("Port {} is open", port),
                //error handling
                Err(e) => println!("Error connecting \
                 to {}: {}", target, e),
                }
        });
    }

    pool.join(); // Waiting for all threads in the pool to complet
}
//clap = "2.33.0"
//threadpool = "1.8.1"
