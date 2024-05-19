use crate::connection::Connection;
use std::net::{TcpListener, TcpStream};


pub struct SimpleServer {
    port: u16,
    listener: TcpListener,
    clients: Vec<Connection>,
    history: Vec<String>,
    sync: usize,
}

impl SimpleServer {
    pub fn new(port: u16) -> SimpleServer {
        SimpleServer {
            port,
            listener: TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Couldn't use port"),
            clients: Vec::new(),
            history: Vec::new(),
            sync: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            //will have to make nonblocking...
            match self.listener.accept() {
                Ok(new_conn) => self.welcome(new_conn),
                Err(e) => println!("{}", e),
            };
            self.handle_clients();

        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn addr(&self) -> String {
        self.listener.local_addr().unwrap().to_string()
    }

    fn welcome(&mut self, new_conn: (TcpStream, std::net::SocketAddr)) {
        let mut new_client = Connection::new(new_conn);
        new_client.send_welcome(&self.history);
        self.clients.push(new_client);
    }

    fn handle_clients(&mut self) {
        for client in self.clients.iter_mut() {
            let delta  = self.sync - client.get_sync();
            if delta > 0 {
                for i in (0..delta).rev() {
                    client.send(self.history[i].clone());
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{net::TcpStream, thread::{self, sleep}, time::Duration};

    use super::SimpleServer;

    #[test]
    fn port_works() {
        let server = SimpleServer::new(42069);
        assert_eq!(42069, server.port());
    }

    #[test]
    #[should_panic]
    fn port_taken() {
        let server1 = SimpleServer::new(42069);
        let server2 = SimpleServer::new(42069);
    }

    #[test]
    fn single_client_can_connect() {
        let mut server = SimpleServer::new(42069);
        thread::spawn(|| -> bool {
            sleep(Duration::from_secs(1));
            let conn = TcpStream::connect("127.0.0.1:42069").expect("WiiWaaWiiWaa");
            true
        });
        server.run();
    }
}
