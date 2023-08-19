use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4};

use tokio::io::AsyncReadExt;
use tokio::net::tcp::OwnedReadHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::models::{ClientSignal, Message, User};

#[derive(Default)]
struct ServerState {
    messages: Vec<Message>,
    users: Vec<User>,
}

pub async fn init() -> tokio::io::Result<()> {
    let server = ServerState::default();
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5555)).await?;

    let (broadcast_sender, broadcast_receiver) = broadcast::channel(128);
    let (signal_sender, signal_receiver) = mpsc::channel(128);

    tokio::spawn(run_broadcast_task(
        broadcast_sender,
        signal_receiver,
        server,
    ));

    loop {
        let (stream, address) = listener.accept().await?;
        tokio::spawn(handle_user_connection(
            stream,
            address,
            broadcast_receiver.resubscribe(),
            signal_sender.clone(),
        ));
    }
}

async fn run_broadcast_task(
    broadcast_sender: broadcast::Sender<Message>,
    mut signal_receiver: mpsc::Receiver<ClientSignal>,
    mut server: ServerState,
) {
    loop {
        let Some(received_signal) = signal_receiver.recv().await else {
            println!("All users have disconnected, ending broadcast task.");
            return;
        };

        use ClientSignal::*;
        match received_signal {
            LogIn(username) => {
                server.users.push(User::new(username.clone()));
                println!("BROADCAST: User {} has logged on.", username);
            }
            LogOut(username) => {
                server.users.retain(|user| user.name != username);
                println!("BROADCAST: User {} has logged out.", username);
            }
            SendMessage(message) => {
                if !server.users.contains(&message.author) {
                    println!(
                        "BROADCAST: User identifying as {} attempted to send a message without logging on.",
                        message.author.name
                    );
                    continue;
                }

                println!(
                    "BROADCAST: User {} has sent a message.",
                    message.author.name
                );
                server.messages.push(message.clone());
                let _ = broadcast_sender.send(message);
            }
        }
    }
}

async fn handle_user_connection(
    stream: TcpStream,
    address: SocketAddr,
    broadcast_receiver: broadcast::Receiver<Message>,
    signal_sender: mpsc::Sender<ClientSignal>,
) -> tokio::io::Result<()> {
    println!("CLIENT HANDLER: Received user connection from {}", address);

    let (mut stream_reader, _stream_writer) = stream.into_split();

    // The first signal must be a LogIn signal.
    let Some(ClientSignal::LogIn(username)) = read_signal(&mut stream_reader).await else {
        // TODO: Move this to broadcast task
        println!(
            "CLIENT HANDLER: User {} attempted to send a signal without first sending a LogIn signal, disconnecting.",
            address
        );

        return Ok(());
    };

    println!("CLIENT HANDLER: User {} has logged on.", username);

    signal_sender
        .send(ClientSignal::LogIn(username.clone()))
        .await
        .unwrap();

    // Listen for signals from the user and broadcasts from the server.
    tokio::spawn(read_broadcasts(username.clone(), broadcast_receiver));
    read_signals(stream_reader, signal_sender).await;

    println!(
        "CLIENT HANDLER: User {} disconnected from {}",
        username, address
    );
    Ok(())
}

async fn read_broadcasts(username: String, mut broadcast_receiver: broadcast::Receiver<Message>) {
    while let Ok(_message) = broadcast_receiver.recv().await {
        println!("CLIENT HANDLER: Broadcasting message to user {}", username);
    }
}

async fn read_signals(mut stream: OwnedReadHalf, signal_sender: mpsc::Sender<ClientSignal>) {
    while let Some(signal) = read_signal(&mut stream).await {
        match &signal {
            // TODO: Error if user is not logged on with this name
            ClientSignal::LogOut(_) => {
                signal_sender.send(signal).await.unwrap();
                break;
            }
            _ => signal_sender.send(signal).await.unwrap(),
        }
    }
}

async fn read_signal(stream: &mut OwnedReadHalf) -> Option<ClientSignal> {
    // If this fails, it means the user has hung up on the connection.
    let signal_size = stream.read_u64().await.ok()?;
    println!("CLIENT HANDLER: Received signal of size {}", signal_size);
    let mut signal_body_buffer = vec![0; signal_size as usize];
    let read_result = stream.read_exact(&mut signal_body_buffer).await;

    match read_result {
        // If the bytes have been read, this indicates that the user has sent a signal.
        Ok(_) => Some(serde_json::from_slice(&signal_body_buffer).unwrap()),
        // If an error has been returned, this indicates that the user has disconnected.
        Err(_) => None,
    }
}
