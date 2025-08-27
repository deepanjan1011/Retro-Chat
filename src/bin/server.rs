use tokio::{
    net::{TcpListener, TcpStream},
    sync::broadcast,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};
//tokio is an async runtime
// async programming do perform others tasks while waiting for one task to finish

use serde::{Serialize,Deserialize};
use chrono::Local;
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
//the above is an attribute - telling the rust compiler to auto generate 4 traits

struct ChatMessage{
    username: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum MessageType {
    UserMessage,
    SystemNotification,
}


#[tokio::main]//it makes the main async func

//main function is sync function - blocking I/O
async fn main() -> Result<(),Box<dyn Error>>{
    let listerner = TcpListener::bind("127.0.0.1:8082").await?;
    println!("╔════════════════════════════════════════╗");
    println!("║        RETRO CHAT SERVER ACTIVE        ║");
    println!("║        Port: 8082  Host: 127.0.0.1     ║");
    println!("║        Press Ctrl+C to shutdown        ║");
    println!("╚════════════════════════════════════════╝");

    let(tx, _ ) = broadcast::channel::<String>(100);
    loop{
        let(socket, addr) = listerner.accept().await?;
        //display connection info
        println!("┌─[{}] New connection", Local::now().format("%H:%M:%S"));
        println!("└─ Address: {}", addr);
        //clone sender for this connection and subscribe a receiver
        let tx = tx.clone();
        let rx = tx.subscribe();

        tokio::spawn(async move {
            handle_connection(socket, tx, rx).await
        });

    }
}
async fn handle_connection(
    mut socket: TcpStream,               // The TCP connection for the client
    tx: broadcast::Sender<String>,      // Sender for broadcasting messages
    mut rx: broadcast::Receiver<String>, // Receiver for incoming broadcasts
) {
    // Split the socket into reader and writer parts
    let (reader, mut writer) = socket.split();
    let mut reader = BufReader::new(reader); // Buffer the reader for efficient I/O
    let mut username = String::new(); // Store the username sent by the client

    // Read the username sent by the client
    reader.read_line(&mut username).await.unwrap();
    let username = username.trim().to_string(); // Remove extra spaces or newlines

    // Send a system notification indicating the user has joined
    let join_msg = ChatMessage {
        username: username.clone(),
        content: "joined the chat".to_string(),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::SystemNotification,
    };
    let join_json = serde_json::to_string(&join_msg).unwrap();
    tx.send(join_json).unwrap();

    // Initialize a buffer for incoming messages from the client
    let mut line = String::new();
    loop {
        tokio::select! {
            // Handle messages sent by the client
            result = reader.read_line(&mut line) => {
                if result.unwrap() == 0 {
                    break; // Exit loop if the client disconnects
                }
                // Create and broadcast a user message
                let msg = ChatMessage {
                    username: username.clone(),
                    content: line.trim().to_string(),
                    timestamp: Local::now().format("%H:%M:%S").to_string(),
                    message_type: MessageType::UserMessage,
                };
                let json = serde_json::to_string(&msg).unwrap();
                tx.send(json).unwrap();
                line.clear(); 
            }
            // Handle incoming broadcasts and send them to the client
            result = rx.recv() => {
                let msg = result.unwrap();
                writer.write_all(msg.as_bytes()).await.unwrap();
                writer.write_all(b"\n").await.unwrap();
            }
        }
    }

    // Send a system notification indicating the user has left
    let leave_msg = ChatMessage {
        username: username.clone(),
        content: "left the chat".to_string(),
        timestamp: Local::now().format("%H:%M:%S").to_string(),
        message_type: MessageType::SystemNotification,
    };
    let leave_json = serde_json::to_string(&leave_msg).unwrap();
    tx.send(leave_json).unwrap();
    
    // Log disconnection information
    println!("└─[{}] {} disconnected", Local::now().format("%H:%M:%S"), username);
}