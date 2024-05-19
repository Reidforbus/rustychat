use crate::connection::Connection;
use std::net::{TcpListener, TcpStream};
use crate::threadpool::ThreadPool;

pub struct TPServer {
    port: u16,
    connections: Vec<Connection>,
    listener: TcpListener,
    pool: ThreadPool,
}

impl TPServer {
    pub fn new(port: u16, threads: usize) -> TPServer {
        TPServer {
            port,
            connections: Vec::new(),
            listener: TcpListener::bind(format!("127.0.0.1:{}", port)).expect("could not bind to port"),
            pool: ThreadPool::new(threads),

        }
    }

    pub fn run(&self) {
        for conn in self.listener.incoming() {
            match conn {
                Ok(conn) => {
                    self.pool.execute(|| handle_client(conn));
                },
                Err(_) => (),
            }
        }
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn addr(&self) -> String {
        self.listener.local_addr().unwrap().to_string()
    }

}


fn handle_client(_tcp: TcpStream) {
}
