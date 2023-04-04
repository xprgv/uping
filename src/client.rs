use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use crate::packet;
use crate::utils::get_now_ns;

#[derive(Default)]
pub struct Client {
    pkts: Arc<Mutex<HashMap<u32, packet::Packet>>>,
}

impl Client {
    pub async fn ping(
        self,
        target_addr: &SocketAddr,
        target_name: &str,
        count: u32,
        interval: Duration,
    ) -> std::io::Result<()> {
        let socket = tokio::net::UdpSocket::bind("0.0.0.0:0").await?;
        let socket = std::sync::Arc::new(socket);

        let in_socket = socket.clone();
        let pkts = self.pkts.clone();
        let dns_name = target_name.to_owned();

        tokio::spawn(async move {
            let mut buf = [0u8; 1024];

            loop {
                if let Ok((len, raddr)) = in_socket.recv_from(&mut buf).await {
                    if let Ok(reply_pkt) = bincode::deserialize::<packet::Packet>(&buf[..len]) {
                        if let Some(packet) = pkts.lock().unwrap().remove(&reply_pkt.idx) {
                            let rtt = Duration::from_nanos(get_now_ns() - packet.ts);
                            println!(
                                "{} bytes from {} ({}): time = {:?}",
                                len, dns_name, raddr, rtt,
                            );
                        }
                    }
                }
            }
        });

        println!("PING {} {}", target_addr, target_name);

        let mut idx = 0;
        while idx < count || count == 0 {
            let request_pkt = packet::Packet::request(idx);
            let bin = bincode::serialize::<packet::Packet>(&request_pkt).unwrap();

            socket.send_to(&bin, target_addr).await?;

            self.pkts
                .lock()
                .unwrap()
                .insert(request_pkt.idx, request_pkt);

            idx += 1;

            tokio::time::sleep(interval).await;
        }

        Ok(())
    }
}
