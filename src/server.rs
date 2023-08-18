use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};
use std::sync::{Arc, RwLock};

use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};

use crate::models::{ClientSignal, Message, Role, User};

pub async fn init() -> tokio::io::Result<()> {
    let _server = ServerData::default();
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5555)).await?;

    loop {
        let (stream, address) = listener.accept().await?;
        tokio::spawn(handle_user_connection(stream, address));
    }
}

#[derive(Default)]
struct ServerData {
    _messages: Arc<RwLock<Vec<Message>>>,
    _users: Arc<RwLock<HashMap<User, Role>>>,
}

async fn handle_user_connection(
    mut stream: TcpStream,
    address: SocketAddr,
) -> tokio::io::Result<()> {
    println!("Recieved connection from {}", address);

    while let Some(signal) = read_signal(&mut stream).await {
        println!("Recieved signal: {:?}", signal);
    }

    println!("Disconnected from {}", address);
    Ok(())
}

async fn read_signal(stream: &mut TcpStream) -> Option<ClientSignal> {
    // If this fails, it means the user has hung up on the connection.
    let signal_size = stream.read_u64().await.ok()?;

    let mut signal_body_buffer = vec![0; signal_size as usize];
    let read_result = stream.read_exact(&mut signal_body_buffer).await;

    match read_result {
        // If the bytes have been read, this indicates that the user has sent a signal.
        Ok(_) => Some(serde_json::from_slice(&signal_body_buffer).unwrap()),
        // If an error has been returned, this indicates that the user has disconnected.
        Err(_) => None,
    }
}
