use std::net::{Ipv4Addr, SocketAddrV4, TcpStream};
use std::time::Duration;

use crate::models::{ClientSignal, EncapsulatedSignal};

pub fn init(username: String) -> std::io::Result<()> {
    let mut stream = TcpStream::connect(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5555))?;

    send_signal(&mut stream, ClientSignal::LogOn(username.clone()))?;
    std::thread::sleep(Duration::from_millis(5));
    send_signal(
        &mut stream,
        ClientSignal::SendMessage("Hello, world!".to_owned()),
    )?;
    std::thread::sleep(Duration::from_millis(5));
    send_signal(&mut stream, ClientSignal::LogOut(username))?;

    Ok(())
}

fn send_signal(stream: &mut TcpStream, signal: ClientSignal) -> std::io::Result<()> {
    EncapsulatedSignal::client(signal).client_send(stream)
}
