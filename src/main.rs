use std::{net::{TcpStream, TcpListener}, thread, io::{BufReader, BufRead, Write}, fs, str::FromStr, string::ParseError};

use rust_web_socket::ThreadPool;

enum RequestType {
    Get,
    Post,
    Other
}

impl FromStr for RequestType {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "GET" => RequestType::Get,
            "POST" => RequestType::Post,
            _ => RequestType::Other
        })
    }
}

enum Paths {
    Root,
    Other
}

impl FromStr for Paths {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "/" => Paths::Root,
            _ => Paths::Other
        })
    }
}

struct Response {
    status_line: String,
    contents: String,
    length: usize
}

impl Response {
    fn format_response(&self) -> String {
        format!("{}\r\nContent-Length: {}\r\n\r\n{}", self.status_line, self.length, self.contents)
    }
}

fn get_file_contents(file_path: &str) -> Option<(usize, String)> {
    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("unable to open HTML file");
            return None;
        }
    };

    Some((contents.len(), contents))
}

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

    let request_details = http_request[0].split([' ']).collect::<Vec<&str>>();
    let request_type = match RequestType::from_str(request_details[0]) {
        Ok(r_type) => r_type,
        Err(_) => {RequestType::Other},
    };
    let path = match Paths::from_str(request_details[1]) {
        Ok(path) => path,
        Err(_) => Paths::Other,
    };

    let ok_file = get_file_contents("hello.html").unwrap();
    let ok_response = Response {
        status_line: String::from("HTTP/1.1 200 OK"),
        contents: ok_file.1,
        length: ok_file.0
    };

    let bad_file = get_file_contents("404.html").unwrap();
    let bad_response = Response {
        status_line: String::from("HTTP/1.1 404 NOT FOUND"),
        contents: bad_file.1,
        length: bad_file.0
    };

    match request_type {
        RequestType::Get => {
            match path {
                Paths::Root => stream.write_all(ok_response.format_response().as_bytes()).unwrap(),
                Paths::Other => stream.write_all(bad_response.format_response().as_bytes()).unwrap(),
            }
        },
        RequestType::Post => stream.write_all(bad_response.format_response().as_bytes()).unwrap(),
        RequestType::Other => stream.write_all(bad_response.format_response().as_bytes()).unwrap(),
    }

    // println!("{:?}",response);
    
}
fn main() {

    let pool = ThreadPool::new(4);
    match TcpListener::bind("127.0.0.1:8080") {
        Ok(listener) => {
            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => {
                        println!("Connection established");
                        pool.execute(|| {
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
