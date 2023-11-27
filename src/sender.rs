use std::net::UdpSocket;

use crate::{message::Message, parse_args::Args};

pub fn send(query: Message)-> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8000")?;  //todo：怎么知道本地用哪个
    socket.connect("127.0.0.53:53")?; //according to ubuntu's /etc/resolv.conf

    let encoded: Vec<u8> = query.try_into().unwrap();
    socket.send(&encoded)?;

    let mut buffer: [u8; 1500] = [0u8; 1500];
    socket.recv_from(&mut buffer)?;

    println!(
        "recv: {:?}",
        buffer
    );
    Ok(())
}


#[test]
fn test_udp_client() {
    let msg = Message::new_query(100, Args::new());
    send(msg);
}