use std::result::Result as StdResult;

use anyhow::Result;
use log::trace;
use tokio::net::{ToSocketAddrs, UdpSocket};

// https://en.wikipedia.org/wiki/Wake-on-LAN#Magic_packet
const HEADER: [u8; 6] = [0xFF; 6];
const MAC_SIZE_BYTES: usize = 6;
const MAC_PER_PACKET: usize = 16;
const PACKET_SIZE_BYTES: usize = 102;
const MAC_SEPARATOR: char = ':';

pub struct Wol {
    packet: Vec<u8>,
}

impl Wol {
    pub fn from_str(mac_address: &str) -> Result<Self> {
        let parsed_mac = Self::parse_mac(mac_address)?;
        let packet = Self::build_packet(&parsed_mac);
        Ok(Wol { packet })
    }

    pub async fn send<A>(&self, destination: A) -> Result<()>
    where
        A: ToSocketAddrs,
    {
        let mut socket = UdpSocket::bind("0.0.0.0:0").await?;
        socket.set_broadcast(true)?;
        socket.send_to(&self.packet, destination).await?;
        Ok(())
    }

    fn parse_mac(mac_address: &str) -> Result<Vec<u8>> {
        let result: StdResult<Vec<u8>, _> = mac_address
            .split(MAC_SEPARATOR)
            .map(|chr| u8::from_str_radix(chr, 16))
            .collect();

        let result = result?;
        if result.len() != MAC_SIZE_BYTES {
            return Err(anyhow::anyhow!(
                "Wrong size mac address {}",
                mac_address.len()
            ));
        }

        trace!("Parsed mac {:?} {:x?}", result, result);
        Ok(result)
    }

    fn build_packet(mac_address: &[u8]) -> Vec<u8> {
        let mut packet = vec![];
        packet.extend(&HEADER);
        let content: Vec<&u8> = std::iter::repeat(mac_address)
            .take(MAC_PER_PACKET)
            .flatten()
            .collect();
        packet.extend(content);
        assert_eq!(packet.len(), PACKET_SIZE_BYTES);
        packet
    }
}
