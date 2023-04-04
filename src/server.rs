use crate::packet;

#[derive(Default)]
pub struct Server {}

impl Server {
    pub async fn listen(self, addr: std::net::SocketAddr) -> std::io::Result<()> {
        let socket = tokio::net::UdpSocket::bind(addr).await?;

        println!("Listen udp ping on: {}", addr);

        let mut buf = [0u8; 1024];
        loop {
            if let Ok((len, raddr)) = socket.recv_from(&mut buf).await {
                if let Ok(request) = bincode::deserialize::<packet::Packet>(&buf[..len]) {
                    let reply = packet::Packet::reply(request.idx, request.ts);

                    if let Err(e) = socket
                        .send_to(&bincode::serialize(&reply).unwrap(), raddr)
                        .await
                    {
                        eprintln!("failed to send reply: {}", e);
                    }
                }
            }
        }
    }
}
