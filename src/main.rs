use web_server::ThreadPool;
//use std::fs;
use std::net::ToSocketAddrs;
use std::io::{prelude::*, BufReader};
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(10);
    let mut servers = vec![String::from("localhost:8080"), String::from("localhost:8081"), String::from("localhost:8082")];
    let (tx, rx) = mpsc::channel();
    
    let args: Vec<String> = env::args().collect();
    
    thread::spawn(move || {
        
        loop {
            println!("Working on health check....");
            let query = &args[1];
            let servers = health_check(&mut servers);
            println!("{:?}", servers);
            println!("servers are sent");
            tx.send(servers).unwrap();
            //drop(servers);
            thread::sleep(Duration::from_secs(query.parse::<u64>().unwrap()));
        }
    });
    let mut id: f64 = 0.0;
    
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let recieved_servers = rx.recv();
        pool.execute( move|| {
            match recieved_servers {
                Ok(servers) => {
                    println!("{}", id);
                    let server = round_robin(&id, servers);
                    println!("the server we are hitting is: {}", server);
                    handle_connection(stream, server);
                    println!("done with connection");
                },
                Err(error) => {
                    println!("didn't recieve anything: {}", error)
                }
            }
           
            
            
            
            
        });
        id = id + 1.0;  
        
    }

    
}

fn handle_connection(mut response_stream: TcpStream, server: String) {
    let mut addrs_iter = server.to_socket_addrs().unwrap();
    let stream = TcpStream::connect(addrs_iter.next().unwrap());
    match stream {
        Ok(mut stream) => {
            let header = format!("GET / HTTP/1.1\r\nHost: http://{}\r\nConnection: close\r\n\r\n", server);
            let _request = stream.write_all(header.as_bytes()).unwrap();
            let mut content = String::new();   
            let mut buf = String::new();
            let _result = stream.read_to_string(&mut buf).unwrap();
            let lines: Vec<String> = buf.lines().map(String::from).collect();
            for line in &lines[7..]{
                content.push_str(line)
            }
            
            
            let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}", content.len(), content);
    
            response_stream.write_all(response.as_bytes()).unwrap();
            response_stream.flush().unwrap();

            }
            Err(_) => {
                print!("{} is not responding....", server);
            }
        }
}

fn health_check(servers: &mut Vec<String>) -> Vec<String> {
    
    for server in servers.to_owned() {
        let mut addrs_iter = server.to_socket_addrs().unwrap();
        let stream = TcpStream::connect(addrs_iter.next().unwrap());
        match stream {
            Ok(mut stream) => {
                let header = format!("GET / HTTP/1.1\r\nHost: http://{}\r\nConnection: close\r\n\r\n", server);
                let _request = stream.write_all(header.as_bytes()).unwrap();         

                let buf = BufReader::new(&mut stream);
                let request_line = buf.lines().next().unwrap().unwrap();
                
                if request_line != "HTTP/1.0 200 OK" {
                    servers.retain(|ser| ser.to_string() == server)

                }
            }
            Err(_) => {
                print!("{} is not responding....", server);
            }
        }
                
        
        
    }
    servers.to_owned()
    
            
    
}

fn round_robin(id: &f64, servers: Vec<String>) -> String {
    
    //let servers = vec![String::from("http://localhost:8080"), String::from("http://localhost:8081"), String::from("http://localhost:8082")];
    let length = servers.len() as f64;
    let index = (id % length).round() as usize;
    
    let server_to_hit = servers.get(index).unwrap();
    server_to_hit.to_string()

   
}