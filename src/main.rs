
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;
mod command;
use command::Command;
use std::sync::Arc;
mod store;
mod persistence;
use store::KeyValueStore;


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    println!("Server listening in port 6379");

    let store = Arc::new(KeyValueStore::new());
    store.load("data.json").await?;

    let store_clone = store.clone();
        
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        store_clone.save("data.json").await.unwrap();
        std::process::exit(0);
    });

    loop {
        let (socket, addr) = listener.accept().await?;
        let store = store.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, store).await {
                eprintln!("Error handling client {}: {:?}", addr, e);    
            }  
        });
    }
} 

async fn handle_client(socket: tokio::net::TcpStream, store: Arc<KeyValueStore>) -> anyhow::Result<()> {
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
        
        let command = Command::parse(&line);

        let response_message = match command {
            Command::Get(key) => {
                if let Some(value) = store.get(&key).await {
                    value
                } else {
                    "Error Key not found".to_string()
                }
            }
            Command::Set(key, value) => {
                store.set(key, value).await;
                "OK".to_string()
            }
            Command::Delete(key) => {
                match store.delete(&key).await {
                    Ok(true) => "Ok".to_string(),
                    Ok(false) => "Key not found".to_string(),
                    Err(e) => "Error occuered".to_string()
                }
            }
            Command::Unknown => "Error Unknown Command".to_string()
        };
        
        let response_message = response_message + "\n";
        
        writer.write_all(response_message.as_bytes()).await?;
    }

    Ok(())
}