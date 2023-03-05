use std::{net::{TcpStream, TcpListener}, thread, io::{BufReader, BufRead, Write}, fs};


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // println!("{:?}", buf_reader);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| match result {
            Ok(result) => result,
            Err(e) => {
                eprintln!("{e}");
                String::from("")
            },
        })
        .take_while(|line| !line.is_empty())
        .collect();

    // println!("Request: {:#?}", http_request);
    let status_line = "HTTP/1.1 200 OK";
    let contents = match fs::read_to_string("hello.html") {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("unable to open HTML file");
            return;
        }
    };

    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    // println!("{:?}",response);
    stream.write_all(response.as_bytes()).unwrap();
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
        Err(e) => eprintln!("Unable to bind to socket: {}", e),
    };
}
