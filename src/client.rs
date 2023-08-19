use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};

use crate::models::{ClientSignal, EncapsulatedSignal};

pub struct Client {
    pub username: String,
    stream: Option<TcpStream>,
}

impl Client {
    pub fn new(username: String) -> Self {
        Self {
            username: username.to_owned(),
            stream: None,
        }
    }

    pub fn connect(&mut self) -> std::io::Result<()> {
        let stream = TcpStream::connect(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5555)).unwrap();
        self.stream = Some(stream);
        self.log_in()
    }

    pub fn disconnect(mut self) -> std::io::Result<()> {
        self.log_out()
    }

    fn log_in(&mut self) -> std::io::Result<()> {
        if let Some(stream) = &mut self.stream {
            send_signal(stream, ClientSignal::LogIn(self.username.clone()))?;
        }

        Ok(())
    }

    fn log_out(&mut self) -> std::io::Result<()> {
        if let Some(stream) = &mut self.stream {
            send_signal(stream, ClientSignal::LogOut(self.username.clone()))?;
        }

        Ok(())
    }

    pub fn send_message(&mut self, content: String) -> std::io::Result<()> {
        if let Some(stream) = &mut self.stream {
            send_signal(stream, ClientSignal::message(&self.username, content))?;
        }

        Ok(())
    }
}

fn send_signal(stream: &mut TcpStream, signal: ClientSignal) -> std::io::Result<()> {
    EncapsulatedSignal::client(signal).client_send(stream)
}
