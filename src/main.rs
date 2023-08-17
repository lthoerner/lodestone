use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::net::{Ipv4Addr, SocketAddrV4};

use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 5555)).await?;
    let mut connection_count: usize = 1;
    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(handle_connection(stream, connection_count));
        connection_count += 1;
    }
}

async fn handle_connection(stream: TcpStream, count: usize) {
    println!("Received connection #{}", count);

    let output_file = File::options()
        .create(true)
        .append(true)
        .open(format!("./output/connection_{}.txt", count))
        .unwrap();

    let mut writer = BufWriter::new(&output_file);
    let mut buffer = [0u8; 1024];
    loop {
        stream.readable().await.unwrap();
        match stream.try_read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => writer.write_all(&buffer).unwrap(),
            Err(e) if e.kind() == io::ErrorKind::WouldBlock => (),
            Err(_) => break,
        }
    }

    println!("Dropped connection #{}", count);
}
