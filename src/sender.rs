use std::net::UdpSocket;

use crate::message::Message;

// pub fn send(query: Message)-> std::io::Result<()> {
//     let socket = UdpSocket::bind("127.0.0.1:8000")?;  //todo：怎么知道本地用哪个
//     socket.connect("127.0.0.1:8089")?; //todo：怎么知道send给谁？

//     socket.send(query.as_bytes())?;
//     let socket = UdpSocket::bind("127.0.0.1:8089")?;

//     let mut buffer = [0u8; 1500];
//     socket.recv_from(&mut buffer)?;
// }