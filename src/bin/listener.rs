use std::{
    io::Read,
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080")
        .expect("could not establish a connection with the address");
    println!("listener ready at port http://localhost:8080");
    match listener.accept() {
        Ok((stream, addr)) => {
            println!("new connection from addr {}", addr);
            print_contents_from_request(stream);
        }
        Err(e) => eprintln!("Error in connecting new client {}", e),
    }
}

fn print_contents_from_request(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // initialising 1kb buffer
    match stream.read(&mut buffer) {
        Ok(bytes_read) => {
            println!("request recieved!");
            if bytes_read > 0 {
                println!("{}", String::from_utf8_lossy(&buffer[..bytes_read]));
            } else {
                println!("empty requst from the client")
            }
        }
        Err(e) => eprintln!("error in reading the request {}", e),
    }
}
