use std::{
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
    thread
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:6379")?;

    println!("Server listening on 127.0.0.1:6379");

    for stream in listener.incoming() {
        let stream = stream?;
        println!("Client connected: {:?}", stream.peer_addr()?);
        thread::spawn(|| {
            handle_connection(stream);
        });
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write_all(response.as_bytes()).unwrap();
    // println!("Request: {http_request:#?}");
}