use async_std::{
    net::TcpListener,
    io::{ ReadExt,WriteExt },
    task::{self, spawn},
};
use futures::stream::StreamExt;
use futures::{AsyncRead, AsyncWrite};
use std::{fs, str, time::Duration};

#[async_std::main]
async fn main() {
    let listener = TcpListener::bind("127.1:8000").await.unwrap();
    listener
        .incoming()
        .for_each_concurrent(None, |stream| async move {
            let stream = stream.unwrap();
            spawn(handle_connection(stream));
        })
        .await;
}

async fn handle_connection(mut stream: impl AsyncRead + AsyncWrite + Unpin) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).await.unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    print!("{}", str::from_utf8(&buffer).unwrap());

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else if buffer.starts_with(sleep) {
        task::sleep(Duration::from_secs(4)).await;
        ("HTTP/1.1 200 OK\r\n\r\n", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();

    let response = format!("{status_line}{contents}");
    stream.write_all(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
