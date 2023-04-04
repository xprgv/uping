use crate::utils::get_now_ns;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum PacketType {
    Request,
    Reply,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Packet {
    pub idx: u32,
    pub packet_type: PacketType,
    pub ts: u64,
}

impl Packet {
    pub fn request(idx: u32) -> Self {
        Self {
            idx,
            packet_type: PacketType::Request,
            ts: get_now_ns(),
        }
    }

    pub fn reply(idx: u32, ts: u64) -> Self {
        Self {
            idx,
            packet_type: PacketType::Reply,
            ts,
        }
    }
}
