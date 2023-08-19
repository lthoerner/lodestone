use std::io::Write;

use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;

#[derive(Serialize, Deserialize, Debug)]
pub struct EncapsulatedSignal {
    pub size: u64,
    pub signal_body: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientSignal {
    LogIn(String),
    LogOut(String),
    SendMessage(Message),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerSignal {
    PropagateMessage(String),
}

impl ClientSignal {
    pub fn message(sender: &str, content: String) -> Self {
        Self::SendMessage(Message::new(User::new(sender.to_owned()), content))
    }
}

impl EncapsulatedSignal {
    pub fn client(signal: ClientSignal) -> Self {
        let signal_body = serde_json::to_vec(&signal).unwrap();
        let size = signal_body.len() as u64;
        Self { size, signal_body }
    }

    pub fn server(signal: ServerSignal) -> Self {
        let signal_body = serde_json::to_vec(&signal).unwrap();
        let size = signal_body.len() as u64;
        Self { size, signal_body }
    }

    pub fn client_send(&self, stream: &mut std::net::TcpStream) -> std::io::Result<()> {
        println!("CLIENT: Sending signal of size {}", self.size);
        stream.write_all(&self.size.to_be_bytes())?;
        stream.write_all(&self.signal_body)?;
        Ok(())
    }

    pub async fn server_send(&self, stream: &mut tokio::net::TcpStream) -> tokio::io::Result<()> {
        stream.write_all(&self.size.to_be_bytes()).await?;
        stream.write_all(&self.signal_body).await?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Message {
    pub author: User,
    pub content: String,
}

impl Message {
    pub fn new(author: User, content: String) -> Self {
        Self { author, content }
    }
}

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Hash, Debug)]
pub struct User {
    pub name: String,
}

impl User {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}
