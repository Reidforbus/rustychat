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
                Err(e) => (),
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

pub struct Server {
    port: u16,
    listener: TcpListener,
}

impl Server {
    pub fn new(port: u16) -> Server {
        Server {
            port,
            listener: TcpListener::bind(format!("127.0.0.1:{}", port)).expect("Could use port"),
        }
    }

    pub fn run(&self) {
        for conn in self.listener.incoming() {
            let conn = conn.unwrap();
            handle_client(conn);
        }
    }
}


fn handle_client(tcp: TcpStream) {
}
