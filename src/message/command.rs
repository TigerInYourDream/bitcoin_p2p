use bitcoin::consensus::{Encodable, encode, Decodable};
use std::{io, iter};


/// Serializer for command string
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct CommandString(pub String);

impl Encodable for CommandString {
    #[inline]
    fn consensus_encode<S: io::Write>(
        &self,
        s: S,
    ) -> Result<usize, encode::Error> {
        let &CommandString(ref inner_str) = self;
        let mut rawbytes = [0u8; 12];
        let strbytes = inner_str.as_bytes();
        if strbytes.len() > 12 {
            panic!("Command string longer than 12 bytes");
        }
        for x in 0..strbytes.len() {
            rawbytes[x] = strbytes[x];
        }
        rawbytes.consensus_encode(s)
    }
}

impl Decodable for CommandString {
    #[inline]
    fn consensus_decode<D: io::Read>(d: D) -> Result<Self, encode::Error> {
        let rawbytes: [u8; 12] = Decodable::consensus_decode(d)?;
        let rv = iter::FromIterator::from_iter(
            rawbytes
                .iter()
                .filter_map(|&u| if u > 0 { Some(u as char) } else { None })
        );
        Ok(CommandString(rv))
    }
}