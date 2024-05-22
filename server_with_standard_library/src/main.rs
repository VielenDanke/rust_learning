use std::{fs, thread};
use std::io::{BufRead, BufReader, Error, Read, Write};
use std::net::{TcpListener, TcpStream};

fn read_file(filename: &str) -> Result<String, Error> {
    Ok(format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", fs::read_to_string(filename)?))
}

fn get_response(request_line: &str) -> String {
    match request_line {
        "GET / HTTP/1.1" => read_file("index.html"),
        "GET /second HTTP/1.1" => read_file("second.html"),
        _ => read_file("404.html")
    }.unwrap_or_else(|e| format!("HTTP/1.1 500 Internal Server Error\r\n\r\n500 Internal Server Error {:?}", e))
}

fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let response = get_response(&request_line);

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to connection: {}", e);
        return;
    }
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush connection: {}", e);
    }
}

fn main() {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr).expect("Failed to bind");

    println!("Server is listening on {}", addr);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }
}
