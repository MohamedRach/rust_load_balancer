use web_server::ThreadPool;
//use std::fs;
use std::net::ToSocketAddrs;
use std::io::{prelude::*, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
use std::thread;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    let args: Vec<String> = env::args().collect();
    
    thread::spawn(move || {
        loop {
            let query = &args[1];
            let request_line = health_check("name");
            println!("{}", request_line);
            thread::sleep(Duration::from_secs(query.parse::<u64>().unwrap()));
        }
    });
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        //health_check();
        pool.execute(|| {
            
            handle_connection(stream, String::from("http://localhost:8080"));
            
            
        });
    }

    
}

fn handle_connection(mut stream: TcpStream, server: String) {
    
    
    let response = format!("HTTP/1.1 301 Moved Permanently\r\nLocation: {}\r\n\r\n", server);

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    
    

}

fn health_check(name: &str) -> String {
    println!("{}: It's time!", name);
    let mut addrs_iter = "localhost:8080".to_socket_addrs().unwrap();
    let mut stream = TcpStream::connect(addrs_iter.next().unwrap()).unwrap();
            
    let header = format!("GET / HTTP/1.1\r\nHost: http://localhost:8080\r\nConnection: close\r\n\r\n");
    let _request = stream.write_all(header.as_bytes()).unwrap();         

    // Make request and return response as string
    let buf = BufReader::new(&mut stream);
    let request_line = buf.lines().next().unwrap().unwrap();
            
    request_line
}