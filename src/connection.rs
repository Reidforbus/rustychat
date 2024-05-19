use std::{io::Write, net::{SocketAddr, TcpStream}};


pub struct Connection {
    tcp: TcpStream,
    sync: usize,
    addr: SocketAddr,
}

impl Connection {
    pub fn new(new_conn: (TcpStream, SocketAddr)) -> Connection {
        Connection {
            tcp: new_conn.0,
            addr: new_conn.1,
            sync: 0,
        }
    }

    pub fn send(&mut self, msg: String) {
        self.tcp.write(&msg.as_bytes()).expect("Sending message failed");
        self.sync += 1;
    }

    pub fn send_welcome(&mut self, msgs: &Vec<String>) {
        for msg in msgs.iter() {
            self.tcp.write(&msg.as_bytes()).expect("Sending message failed");
            self.sync += 1;
        }
    }

    pub fn get_sync(&self) -> usize {
        self.sync
    }

    pub fn get_addr(&self) -> SocketAddr {
        self.addr
    }
}

