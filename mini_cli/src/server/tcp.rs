use std::net::TcpListener;

use crate::{
    core::request::parse_params,
    router::{ router::route},
};

fn handle_connection(mut stream: std::net::TcpStream) {
    use std::io::{Read, Write};
    let mut buffer = [0; 4096];
    match stream.read(&mut buffer) {
        Ok(size) => {
            let request_str = String::from_utf8_lossy(&buffer[..size]);

            let parsed_request = parse_params(&request_str);
            
            let response = route(&parsed_request);
            
            let response_str = format!(
                "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
                response.status_code,
                response.body.len(),
                response.body
            );
            
            stream
                .write(response_str.as_bytes())
                .expect("Failed to write response");
        }
        Err(e) => {
            eprintln!("Failed to read from connection: {}", e);
        }
    }
}

pub fn start_server(addr: &str) {
    println!("Starting TCP server on {}", addr);
    let listener = TcpListener::bind(addr).expect("Cannot bind");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
}
