use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.1:8000").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established");
    }
}
