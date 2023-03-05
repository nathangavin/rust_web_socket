use std::{net::{TcpStream, TcpListener}, thread, io::{Read, Write, BufReader, BufRead}};


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("Request: {:#?}", http_request);
}
fn main() {
    match TcpListener::bind("127.0.0.1:8080") {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("Connection established");
                        thread::spawn(|| {
                            handle_connection(stream);
                        });
                    },
                    Err(e) => eprintln!("Connection failed: {}", e),
                }
            }
        },
        Err(e) => eprintln!("Unabled to bind to socket: {}", e),
    };
}
