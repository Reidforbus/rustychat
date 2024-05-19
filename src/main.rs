fn main() {
    let mut server = rustychat::server::SimpleServer::new(42069);
    println!("New server created, listening to address: {}", server.addr());
    server.run();
}
