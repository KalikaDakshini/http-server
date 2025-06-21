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
        println!("Server online\nIP: {}", self.addr);
    }
}
