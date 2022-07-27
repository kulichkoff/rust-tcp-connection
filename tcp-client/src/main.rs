use std::net::{TcpStream};
use std::io::{Read, Write, stdin, stdout};
use std::str::from_utf8;

fn main() {
    match TcpStream::connect("localhost:8989") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 8989");

            let mut text = String::new();
            stdout().flush().unwrap();
            stdin().read_line(&mut text).expect("Did not enter a correct string");

            let msg = text.as_bytes();

            stream.write(msg).unwrap();
            let reply_msg = from_utf8(msg).unwrap();
            println!("Sent {}, awaiting reply...", reply_msg);

            let mut data = [0 as u8; 50]; // using 6 bytes buffer;

            match stream.read_exact(&mut data) {
                Ok(_) => {
                    let msg = from_utf8(&data).unwrap();
                    println!("Reply from server: {}", msg);
                },
                Err(e) => {
                    println!("Unexpected error: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to recieve data: {}", e);
        }
    }

    println!("Client was terminated");
}
