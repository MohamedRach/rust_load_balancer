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
    //let mut streams = HashMap::new();

    
    let args: Vec<String> = env::args().collect();
    
    thread::spawn(move || {
        loop {
            let query = &args[1];
            let request_line = health_check("name");
            println!("{}", request_line);
            thread::sleep(Duration::from_secs(query.parse::<u64>().unwrap()));
        }
    });
    let mut id: f64 = 0.0;
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(move || {
            let server = round_robin(&id);
            println!("{}", server);
            handle_connection(stream, server);
            
            
        });
        id = id + 1.0;
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

    let buf = BufReader::new(&mut stream);
    let request_line = buf.lines().next().unwrap().unwrap();
            
    request_line
}

fn round_robin(id: &f64) -> String {
    let servers = vec![String::from("http://localhost:8080"), String::from("http://localhost:8081"), String::from("http://localhost:8082")];
    let length = servers.len() as f64;
    let index = (id % length).round() as usize;

    let server_to_hit = servers.get(index).unwrap();

    server_to_hit.to_string()
}