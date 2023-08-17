use std::collections::HashMap;
use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::{Arc, RwLock};

use tokio::net::{TcpListener, TcpStream};

use crate::models::{Role, User};

pub async fn init() -> tokio::io::Result<()> {
    let server = ServerData::default();
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5555)).await?;

    loop {
        let (stream, _) = listener.accept().await.unwrap();
        // tokio::spawn(server.handle_user(stream));
    }

    todo!()
}

#[derive(Default)]
struct ServerData {
    channels: Arc<RwLock<HashMap<String, Vec<String>>>>,
    users: Arc<RwLock<HashMap<User, Role>>>,
}

impl ServerData {
    async fn handle_user(&mut self, stream: TcpStream) {
        todo!()
    }
}
