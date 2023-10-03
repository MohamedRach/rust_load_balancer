use web_server::ThreadPool;
//use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
//use std::thread;
//use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    
}

fn handle_connection(mut stream: TcpStream) {
    

    let response = format!("HTTP/1.1 301 Moved Permanently\r\nLocation: http://localhost:8080/\r\n\r\n");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}