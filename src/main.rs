use std::net::TcpListener;

use web_server::EventLoop;
fn main() {
   std::env::set_var("RUST_BACKTRACE", "1");
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap(); // start listening for tcp connection on port 7878
    EventLoop::eventLoop(listener);
}

