
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Server listening in port 6379");

    loop {
        let (socket, addr) = listener.accept().await?;
        
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket).await {
                eprintln!("Error handling client {}: {:?}", addr, e);    
            }  
        });
    }
} 

async fn handle_client(socket: tokio::net::TcpStream) -> anyhow::Result<()> {
    let (reader, mut writer) = socket.into_split();
    let mut reader = BufReader::new(reader);
    let mut line = String::new();
    
    loop {
        line.clear();
        let bytes_read = reader.read_line(&mut line).await?;

        if bytes_read == 0 {
            break;
        }

        println!("Received: {}", line.trim());
        writer.write_all(line.as_bytes()).await?;
    }

    Ok(())
}