use crate::message::address::Address;

/// The `version` message
/// 添加了一个bytes 属性
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct VersionMessage {
    /// The P2P network protocol version
    pub version: u32,
    /// A bitmask describing the services supported by this node
    pub services: u64,
    /// The time at which the `version` message was sent
    pub timestamp: i64,
    /// The network address of the peer receiving the message
    pub receiver: Address,
    /// The network address of the peer sending the message
    pub sender: Address,
    /// A random nonce used to detect loops in the network
    pub nonce: u64,
    /// bytes in version string
    /// 没有出现在murmel中，也没有出现在bitcoin wiki中，但是出现在了[https://bitcoin.org/en/developer-reference#version]
    pub bytes: u8,
    /// A string describing the peer's software
    pub user_agent: String,
    /// The height of the maximum-work blockchain that the peer is aware of
    pub start_height: i32,
    /// Whether the receiving peer should relay messages to the sender; used
    /// if the sender is bandwidth-limited and would like to support bloom
    /// filtering. Defaults to true.
    pub relay: bool
}

impl VersionMessage {
    /// Constructs a new `version` message
    pub fn new(
        services: u64,
        timestamp: i64,
        receiver: Address,
        sender: Address,
        nonce: u64,
        bytes:u8,
        user_agent: String,
        start_height: i32,
    ) -> VersionMessage {
        VersionMessage {
            //固定值
            version: 70001,
            services: services,
            timestamp: timestamp,
            receiver: receiver,
            sender: sender,
            nonce: nonce,
            bytes,
            user_agent: user_agent,
            start_height: start_height,
            relay: false,
        }
    }
}

//从rust-bitcoin 抄过来 用来处理编码问题，用法把结构体和结构体的每个成员传进去
#[macro_export]
macro_rules! impl_consensus_encoding {
    ($thing:ident, $($field:ident),+) => (
        impl bitcoin::consensus::Encodable for $thing {
            #[inline]
            fn consensus_encode<S: ::std::io::Write>(
                &self,
                mut s: S,
            ) -> Result<usize, bitcoin::consensus::encode::Error> {
                let mut len = 0;
                $(len += self.$field.consensus_encode(&mut s)?;)+
                Ok(len)
            }
        }

        impl bitcoin::consensus::Decodable for $thing {
            #[inline]
            fn consensus_decode<D: ::std::io::Read>(
                mut d: D,
            ) -> Result<$thing, bitcoin::consensus::encode::Error> {
                Ok($thing {
                    $($field: bitcoin::consensus::Decodable::consensus_decode(&mut d)?),+
                })
            }
        }
    );
}

impl_consensus_encoding!(VersionMessage, version, services, timestamp,
                         receiver, sender, nonce, bytes,
                         user_agent, start_height, relay);

