use std::io;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn handle_stream(mut stream: TcpStream) -> io::Result<()> {
    stream.write_all(b"+PONG\r\n")?;
    stream.flush()?;
    Ok(())
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        handle_stream(stream?)?;
    }

    Ok(())
}
