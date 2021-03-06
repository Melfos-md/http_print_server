use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;


fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("[*] Server started on http://127.0.0.1:7878");
    let mut i = 0;
    for stream in listener.incoming() {
        i += 1;
        let stream = stream.unwrap();
        println!("[*] New connection established... connection number: {}", i);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;2048];

    stream.read(&mut buffer).unwrap();
    println!("[>] {}", String::from_utf8_lossy(&buffer[..]));
    let response = "HTTP/1.1 200 OK\r\n\r\n";
    stream.write(response.as_bytes()).unwrap();
}