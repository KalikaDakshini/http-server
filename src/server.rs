use std::io::Read;
use std::net::TcpListener;

use crate::http::Request;

pub struct Server {
    addr: String,
}

impl Server {
    /// Return a new server instace
    pub fn new(ip: String) -> Self {
        Self { addr: ip }
    }

    /// Run the server
    pub fn run(&self) {
        // Set up a TCP Listener
        let listener = TcpListener::bind(&self.addr).expect("Failed to establish a TCP connection");

        // Check for connections
        loop {
            // Try to connect again if error
            let Ok((mut stream, _)) = listener.accept() else {
                println!("Failed to establish connection. Trying again");
                continue;
            };

            // Read the request into a buffer
            let mut buf = [0; 1024];
            if let Err(e) = stream.read(&mut buf) {
                println!("Failed with error {}", e);
            }
            // Parse it into a request
            else {
                let Ok(request) = Request::try_from(&buf[..]) else {
                    println!("Failed to parse request");
                    continue;
                };
                println!("{}", request);
            }
        }
    }
}
