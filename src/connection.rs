use std::net::TcpStream;


pub struct Connection {
    tcp: TcpStream
}

impl Connection {
    pub fn listen(&self) {
        println!("Listening to {:?}", self.tcp.peer_addr());
    }
}

