use rand::random;
use std::{
    net::{Ipv4Addr, UdpSocket},
    time::Duration,
};
use url::Url;

fn dns_resolver(host: &str, dns: &str) -> Result<Vec<Ipv4Addr>, String> {
    let socket =
        UdpSocket::bind("0.0.0.0:0").map_err(|_| String::from("udp socket bidning problem"))?;
    socket
        .set_read_timeout(Some(Duration::from_secs(2)))
        .map_err(|_| String::from("read timeout error"))?;
    socket
        .connect((dns, 53))
        .map_err(|_| String::from("socket connection error"))?;

    let mut packet = Vec::with_capacity(512);

    let txid: u16 = random();

    packet.extend_from_slice(&txid.to_be_bytes());
    packet.extend_from_slice(&0x0100u16.to_be_bytes());
    packet.extend_from_slice(&1u16.to_be_bytes());
    packet.extend_from_slice(&0u16.to_be_bytes());
    packet.extend_from_slice(&0u16.to_be_bytes());
    packet.extend_from_slice(&0u16.to_be_bytes());

    for part in host.split('.') {
        packet.push(part.len() as u8);
        packet.extend_from_slice(part.as_bytes());
    }
    packet.push(0);

    packet.extend_from_slice(&1u16.to_be_bytes());

    packet.extend_from_slice(&1u16.to_be_bytes());

    socket
        .send(&packet)
        .map_err(|_| String::from("packet sending error"))?;

    let mut buf = [0u8; 512];
    let size = socket
        .recv(&mut buf)
        .map_err(|_| String::from("socket receive error"))?;

    let id = u16::from_be_bytes([buf[0], buf[1]]);
    if id != txid {
        return Err(String::from("transaction id mismatch"));
    }

    let answers = u16::from_be_bytes([buf[6], buf[7]]) as usize;

    let mut pos = 12;

    while buf[pos] != 0 {
        pos += buf[pos] as usize + 1;
    }

    pos += 1;
    pos += 4;

    let mut ips = Vec::new();

    for _ in 0..answers {
        if buf[pos] & 0xC0 == 0xC0 {
            pos += 2;
        } else {
            while buf[pos] != 0 {
                pos += buf[pos] as usize + 1;
            }
            pos += 1;
        }

        let typ = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        pos += 2;

        let class = u16::from_be_bytes([buf[pos], buf[pos + 1]]);
        pos += 2;

        pos += 4;

        let rdlen = u16::from_be_bytes([buf[pos], buf[pos + 1]]) as usize;
        pos += 2;

        if typ == 1 && class == 1 && rdlen == 4 {
            ips.push(Ipv4Addr::new(
                buf[pos],
                buf[pos + 1],
                buf[pos + 2],
                buf[pos + 3],
            ));
        }

        pos += rdlen;
    }

    Ok(ips)
}

pub fn resolve_url(url: String, dns: &str) -> Result<Ipv4Addr, String> {
    let mut hostname = String::new();
    let parsed_url = Url::parse(&url).map_err(|_| String::from("error in parsing url"))?;

    if let Some(value) = parsed_url.host_str() {
        hostname.push_str(value);
    }

    let resolved_url = dns_resolver(&hostname, dns)?;
    if resolved_url.is_empty() {
        return Err(String::from("No resolved IP found"));
    }
    Ok(resolved_url[0])
}
