use std::io::{ErrorKind, Read};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;

//create constant for localhost
const LOCAL: &str = "127.0.0.1:8070";
//create constant for message size
const MSG_SIZE: usize = 32;

fn main() {
    // create a server
    
    let server = TcpListener::bind(LOCAL)
        .expect("Listener failed to bind");
    
    // set server to non-blocking
    server.set_nonblocking(true).expect("Failed to initialize non-blocking");
    
    // create a vector of clients
    let mut clients = vec![];
    
    // create a channel for messages
    let (sender, receiver) =
        mpsc::channel::<String>();
    
    loop {
        // handle new connections
        if let Ok((mut socket, addr)) = server.accept() {
            println!("Client {} connected", addr);
            let tx = &sender;
            clients.push(socket.try_clone()
                .expect("Failed to clone client"));
           
            // send message to client
            thread::spawn(move || loop {
                let mut buff = vec![0; MSG_SIZE];
                
                // read message from client
                match socket.read_exact(&mut buff) {
                    Ok(_) => {
                        let msg = buff
                            .into_iter()
                            .take_while(|&x| x != 0)
                            .collect::<Vec<_>>();
                        // convert to string
                        let msg = String::from_utf8(msg)
                            .expect("Invalid utf8 message");
                        println!("{}: {:?}", addr, msg);
                        tx.send(msg).expect("Failed to send msg");
                    },
                    
                    // handle error
                    Err(ref err) if err.kind() ==
                        ErrorKind::WouldBlock => (),
                    Err(_) => {
                        println!("Closing connection with: {}", addr);
                        break;
                    }
                }
            });
        }
    }
}
