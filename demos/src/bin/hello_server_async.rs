use std::io;
use std::sync::Arc;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
    sync::Semaphore,
    time::{Duration, sleep},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:7878").await?;
    let semsphore = Arc::new(Semaphore::new(3));
    loop {
        let (stream, addr) = listener.accept().await?;
        let permit = Arc::clone(&semsphore).acquire_owned().await.unwrap();

        tokio::spawn(async move {
            let _permit = permit;
            if let Err(e) = handle_conn(stream).await {
                eprintln!("[{addr}] error: {e}");
            }
        });
    }
}

async fn handle_conn(stream: TcpStream) -> io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut request_line = String::new();

    reader.read_line(&mut request_line).await?;

    let (status_line, filename) = match request_line.trim_end() {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "demos/src/static/hello.html"),
        "GET /sleep HTTP/1.1" => {
            sleep(Duration::from_secs(5)).await;
            ("HTTP/1.1 200 OK", "demos/src/static/404.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "demos/src/static/404.html"),
    };

    let contents = tokio::fs::read_to_string(filename).await?;
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    let mut stream = reader.into_inner();
    stream.write_all(response.as_bytes()).await?;
    Ok(())
}
