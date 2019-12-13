use bitcoin::consensus::serialize;
#[macro_use]
pub mod version;
pub mod address;
pub mod command;
pub mod filterload;
pub mod getdata;

pub const MAINNET: u32 = 0xF9BEB4D9;
pub const TESTNET: u32 = 0xFABFB5DA;

/// 消息最终发出去的形态
/// https://en.bitcoin.it/wiki/Protocol_documentation#Message_structure
///
/// Message struct
///     magic       NetworkString
///     command     ASCII string identifying the packet content
///     length      Length of payload in number of bytes
///     checksum
///     payload     The actual data
///
///     magic:
///             F9 BE B4 D9 mainnet
///             FA BF B5 DA Testnet
///             0B 11 09 07 testnet3
///             F9 BE B4 FE namecoin
///
///     payload: 具体消息
///
///     magic command payload 这三个为传入属性 其余两个为计算值
///
pub struct RawMessage {
    magic: Magic,
    command: command::CommandString,
    payload: Payload,
}

pub enum Magic {
    Main,
    Testnet,
}

pub enum Payload {
    Version(version::VersionMessage),
    Verack,
    FilterLoad(filterload::FilterLoad),
    GetData(getdata::GetData)
}


impl Payload {
    //计算自己长度, 计算自己的checksum, 序列化自己 有些数据是没有payload的
    pub fn calc(&self) -> (u32, Vec<u8>, Option<Vec<u8>>) {
        match self {
            Payload::Version(data) => {
                let serialize = serialize(data);
                let len = serialize.len();
                let checksum = sha_sha(&serialize);
                (len as u32, checksum, Some(serialize))
            }

            Payload::Verack => {
                //Verack 没有长度 checksum按照空算 payload本身没有
                let checksum = sha_sha("".as_bytes());
                (0, checksum, None)
            }

            Payload::FilterLoad(data) => {
                //先根据固定数据来一个
                //let serialize = serialize(data);
                //let len = serialize.len();
                //let checksum = sha_sha(&serialize);
                //(len as u32, checksum, Some(serialize))
                let serialize = &data.0;
                let len = serialize.len();
                let checksum = sha_sha(&serialize);
                (len as u32, checksum, Some(serialize.to_owned()))

            }

            Payload::GetData(data) => {
                //固定数据
                let serialize = &data.0;
                let len = serialize.len();
                let checksum = sha_sha(&serialize);
                (len as u32, checksum, Some(serialize.to_owned()))
            }
        }
    }
}

//计算checksum 的具体过程 SHA256(SHA256(payload)) 取前四个字节
pub fn sha_sha(input: &[u8]) -> Vec<u8> {
    let hash_m1 = hmac_sha256::Hash::hash(input);
    let hash_m2 = hmac_sha256::Hash::hash(&hash_m1);
    hash_m2[0..4].to_owned()
}

/// A Network message payload. 也就是具体信息
/// [Bitcoin Wiki: Protocol Specification](https://en.bitcoin.it/wiki/Protocol_specification)
/// 注意这份文档中的具体消息可能过时了，但是基本是可以对应上的
///
/// 用法
///     组合具体的payload
///     把network_type+CommandString+payload 传入RawMessage
///     RawMessage进行combine (需要在RawMessage中 计算checksum 和 payload——length 最后发出的数据有5个参数组合起来)
///     全体的序列化都在combine中进行
///
impl RawMessage {
    pub fn new(magic: Magic, command: command::CommandString, payload: Payload) -> Self {
        RawMessage {
            magic,
            command,
            payload,
        }
    }

    //根据magic 选取对应网络类型对应的数字
    pub fn magic_num(&self) -> u32 {
        match self.magic {
            Magic::Main => {
                0xD9B4BEF9
            }
            Magic::Testnet => {
                0xDAB5BFFA
            }
        }
    }

    /// 把序列化好的数据进行拼接 组成完整的需要发送的数据
    pub fn combine(&self) -> Vec<u8> {
        let mut raw_bytes: Vec<u8> = Vec::new();

        let mut magic = serialize(&(self.magic_num()));
        let mut command = serialize(&(self.command));

        let (len, mut checksum, mut payload) = self.payload.calc();
        let mut payload_len: Vec<u8> = serialize(&len);

        raw_bytes.append(&mut magic);
        raw_bytes.append(&mut command);
        raw_bytes.append(&mut payload_len);
        raw_bytes.append(&mut checksum);

        if let Some(ref mut data) = payload {
            raw_bytes.append(data);
        }

        raw_bytes
    }
}
