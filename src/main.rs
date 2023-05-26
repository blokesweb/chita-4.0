use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let request = String::from_utf8_lossy(&buffer[..]);
    let path = get_requested_path(&request);

    let response = match fs::read_to_string(&path) {
        Ok(content) => format!("HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\n\r\n{}", content),
        Err(_) => String::from("HTTP/1.1 404 NOT FOUND\r\nContent-Type: text/plain; charset=utf-8\r\n\r\n404 Not Found"),
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn get_requested_path(request: &str) -> String {
    let lines: Vec<&str> = request.lines().collect();
    let first_line: Vec<&str> = lines[0].split(' ').collect();
    let path = &first_line[1][1..]; // Remove the leading '/'

    // If the requested path is "/", append "index.html" to it
    let path = if path == "" || path == "/" {
        format!("www/index.html")
    } else {
        format!("www/{}", path)
    };

    path
}


fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind address");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to establish a connection: {}", e);
            }
        }
    }
}
