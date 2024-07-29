use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut stream_buffer = [0; 1024];
    stream
        .read(&mut stream_buffer)
        .expect("failed to read into buffer");

    let request = String::from_utf8_lossy(&stream_buffer);
    println!("Your request is {}", request);

    let response = "Hello, Client".as_bytes();
    stream.write(response).expect("Failed to respond to client");
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind to address.");
    println!("Server listening on 127.0.0.1:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("failed to establish connection {}", e);
            }
        }
    }
}
