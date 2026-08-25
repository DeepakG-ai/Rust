use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Bind to an address and port (async)
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server listening on 127.0.0.1:8080");

    // 2. Loop continuously to accept incoming connections
    loop {
        // .accept() waits asynchronously for a new client
        let (mut socket, addr) = listener.accept().await?;
        println!("New client connected: {addr}");

        // 3. Spawn a lightweight Tokio task for each connected client
        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                // Asynchronously read data from the client socket
                let n = match socket.read(&mut buf).await {
                    Ok(0) => return, // 0 bytes means client disconnected
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("Failed to read from socket: {e}");
                        return;
                    }
                };

                // Asynchronously write data back (echo)
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("Failed to write to socket: {e}");
                    return;
                }
            }
        });
    }
}
