use polling::{Events, Poller, Event};
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs,
    thread
};


pub struct EventLoop;

impl EventLoop {
    pub fn eventLoop(socket: TcpListener) {
        let poller = Poller::new().unwrap();
        let key = 7;
        unsafe {
            let _add = poller.add(&socket, Event::readable(key)).unwrap();
        }
        let mut events = Events::new();
        loop{
            events.clear();
            let _wait = poller.wait(&mut events, None).unwrap();
            for ev in events.iter() {
                if ev.key == key {
                    // Perform a non-blocking accept operation.
                    for stream in socket.incoming() { // the incoming method gives us sequence of streams
                        let stream = stream.unwrap();
                        thread::spawn(|| {
                            Self::handle_connection(stream);
                            println!("new thread initiated"); 
                    });
                    }  
                    // Set interest in the next readability event.
                    let _modify = poller.modify(&socket, Event::readable(key)).unwrap();
                } else {
                    println!("not valid key")
                }
            }
        }
    }

    fn handle_connection(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream); //adds buffering by managing calls to the std::io::Read trait methods
        let request_line = buf_reader.lines().next().unwrap().unwrap();
        let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
            ("HTTP/1.1 200 OK", "index.html")
        } else {
            ("HTTP/1.1 404 NOT FOUND", "404.html")
        };
                                    
        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();
        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}