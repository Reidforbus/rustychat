fn main() {
    let server = rustychat::server::TPServer::new(42069, 4);
    println!("New server created, listening to address: {}", server.addr());
    server.run();
}
