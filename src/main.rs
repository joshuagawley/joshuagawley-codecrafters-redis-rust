use std::io;
use std::net::TcpListener;
use tokio::io::AsyncWriteExt;

async fn handle_stream(mut stream: tokio::net::TcpStream) -> io::Result<()> {
    stream.write_all(b"+PONG\r\n").await?;
    stream.flush().await?;

    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("line 19 main");

    for stream in listener.incoming() {
        let stream = stream?;
        stream.set_nonblocking(true)?;
        handle_stream(tokio::net::TcpStream::from_std(stream)?).await?;
    }

    Ok(())
}
