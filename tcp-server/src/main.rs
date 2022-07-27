use std::str::from_utf8;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8989").unwrap();
    println!("Server is listening on localhost:8989");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_client(stream);
                });
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }

    drop(listener);
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // 50 byte buffer;

    while match stream.read(&mut data) {
        Ok(size) => {
            if size == 0 {
                println!("Closing connection with {}", stream.peer_addr().unwrap());
                stream.shutdown(Shutdown::Both).unwrap();
                false
            } else {
                println!("Got message with size {}", size);
                let msg = from_utf8(&data).unwrap();
                println!("Message: {}", msg);
                stream.write(&data[0..size]).unwrap();
                true
            }
        },
        Err(_) => {
            println!("An error occured, terminating connection with {}", stream.peer_addr().unwrap());
            false
        }
    } {}
}
