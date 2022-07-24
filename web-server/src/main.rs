use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

fn main(){
    let listener = TcpListener::bind("localhost:8080").unwrap();


    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established with {}", stream.peer_addr().unwrap());

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream){
    let mut buffer = [0;1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}