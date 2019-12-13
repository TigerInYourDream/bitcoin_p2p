use std::net::{SocketAddr, Ipv6Addr, SocketAddrV4, SocketAddrV6};
use std::{io, fmt};
use bitcoin::consensus::{Encodable, Decodable, encode};

/// A message which can be sent on the Bitcoin network
pub struct Address {
    /// Services provided by the peer whose address this is
    pub services: u64,
    /// Network byte-order ipv6 address, or ipv4-mapped ipv6 address
    pub address: [u16; 8],
    /// Network port
    pub port: u16
}

const ONION : [u16; 3] = [0xFD87, 0xD87E, 0xEB43];

impl Address {
    /// Create an address message for a socket
    pub fn new (socket :&SocketAddr, services: u64) -> Address {
        let (address, port) = match socket {
            &SocketAddr::V4(ref addr) => (addr.ip().to_ipv6_mapped().segments(), addr.port()),
            &SocketAddr::V6(ref addr) => (addr.ip().segments(), addr.port())
        };
        Address { address: address, port: port, services: services }
    }

    /// extract socket address from an address message
    /// This will return io::Error ErrorKind::AddrNotAvailable if the message contains a Tor address.
    pub fn socket_addr (&self) -> Result<SocketAddr, io::Error> {
        let addr = &self.address;
        if addr[0..3] == ONION[..] {
            return Err(io::Error::from(io::ErrorKind::AddrNotAvailable));
        }
        let ipv6 = Ipv6Addr::new(
            addr[0],addr[1],addr[2],addr[3],
            addr[4],addr[5],addr[6],addr[7]
        );
        if let Some(ipv4) = ipv6.to_ipv4() {
            Ok(SocketAddr::V4(SocketAddrV4::new(ipv4, self.port)))
        }
        else {
            Ok(SocketAddr::V6(SocketAddrV6::new(ipv6, self.port, 0, 0)))
        }
    }
}

// to_be 转换成大端序
fn addr_to_be(addr: [u16; 8]) -> [u16; 8] {
    [addr[0].to_be(), addr[1].to_be(), addr[2].to_be(), addr[3].to_be(),
        addr[4].to_be(), addr[5].to_be(), addr[6].to_be(), addr[7].to_be()]
}

//这些方法都是方便使用bitcoin 库中序列化和反序列化的
impl Encodable for Address {
    #[inline]
    fn consensus_encode<S: io::Write>(
        &self,
        mut s: S,
    ) -> Result<usize, encode::Error> {
        let len = self.services.consensus_encode(&mut s)?
            + addr_to_be(self.address).consensus_encode(&mut s)?
            + self.port.to_be().consensus_encode(s)?;
        Ok(len)
    }
}

impl Decodable for Address {
    #[inline]
    fn consensus_decode<D: io::Read>(mut d: D) -> Result<Self, encode::Error> {
        Ok(Address {
            services: Decodable::consensus_decode(&mut d)?,
            address: addr_to_be(Decodable::consensus_decode(&mut d)?),
            port: u16::from_be(Decodable::consensus_decode(d)?)
        })
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: render services and hex-ize address
        write!(f, "Address {{services: {:?}, address: {:?}, port: {:?}}}",
               self.services, &self.address[..], self.port)
    }
}

impl Clone for Address {
    fn clone(&self) -> Address {
        Address {
            services: self.services,
            address: self.address,
            port: self.port,
        }
    }
}

impl PartialEq for Address {
    fn eq(&self, other: &Address) -> bool {
        self.services == other.services &&
            &self.address[..] == &other.address[..] &&
            self.port == other.port
    }
}

impl Eq for Address {}