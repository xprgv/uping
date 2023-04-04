use dns_lookup::lookup_host;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_now_ns() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64
}

fn parse_addr(addr: &str) -> Option<(std::net::IpAddr, String)> {
    if let Ok(ip_addr) = addr.parse::<std::net::IpAddr>() {
        Some((ip_addr, "".to_string()))
    } else {
        // resolve ip addrs
        let ip_addrs = match lookup_host(addr) {
            Ok(ip_addrs) => ip_addrs,
            Err(_) => return None,
        };
        let ip_addr = match ip_addrs.get(0) {
            Some(ip_addr) => ip_addr,
            None => return None,
        };
        Some((*ip_addr, String::from(addr)))
    }
}

pub fn parse_target_addr(addr: &str) -> Option<(std::net::SocketAddr, String)> {
    let parts: Vec<&str> = addr.split(':').collect();
    if parts.len() != 2 {
        return None;
    }

    let addr = parts[0];
    let port = parts[1];

    let (ip_addr, dns_name) = parse_addr(addr)?;

    let port = match port.parse::<u16>() {
        Ok(port) => port,
        Err(_) => return None,
    };

    let socket_addr = std::net::SocketAddr::new(ip_addr, port);
    Some((socket_addr, dns_name))
}
