use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("could not establish a connection with the address");
    match listener.accept() {
        Ok((stream, addr)) => {
            println!("new connection from addr {}", addr);
            print_contents_from_request(stream);
        }
        Err(e) => println!("Error in connecting new client {}", e),
    }
}

fn print_contents_from_request(stream: TcpStream) -> () {
}
