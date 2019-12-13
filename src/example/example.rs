//! the example for how to decode hex string to message struct
//!
//! 转化过程 一个string 利用hex转换成 Vec<u8>
//! u8数组再利用 bitcoin::deserialize 转换成相应的Address类型
//! 要求 Address 实现 Decode和Encode方法 不然无法实现serialize 和 deserialize
//!
//!
fn decode_address() {

    use hex::decode as hex_decode;
    use bitcoin::consensus::{deserialize, serialize};
    use crate::message::address::Address;
    use std::net::{SocketAddr, SocketAddrV4, IpAddr, Ipv4Addr};

    //test for address hex decode to structures
    let from_sat = hex_decode("000000000000000000000000000000000000ffff7f000001208d").unwrap();
    println!("The byte is {:?}", &from_sat);
    let decode: Result<Address, _> = deserialize(&from_sat);
    assert!(decode.is_ok());
    let address = &decode.unwrap();
    println!("The Address is {:?}", address);
}

//! 从一个Address 类型转换为 hex string
//! 先组装类型 然后利用 bitcoin库提供的serialize方法转换为数组 然后利用hex库的encode方法转为hex string
//! serialize方法要求实现Encode trait
//!
//! hex_decode 0xxxxx---->Vec
//! deserialize Vec -----> struct
//!
//! serialize struct ----> Vec
//! hex_encode Vec ------> 0xxxxxxxx
//!
//! vec是在网络上传播的形式
//!
fn encode_address(){
    //test for address to encode hex
    let s4 = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8333);
    let a4 = Address::new(&s4, 0);
    let v4c = serialize(&a4);
    let res = hex::encode(v4c);
    println!("The hex string is {:?}", res);
}

//! 主要流程的拼装数据的流程
fn example(){
    // This message is from my satoshi node, morning of May 27 2014
//    let from_sat = hex_decode("721101000100000000000000e6e0845300000000010000000000000000000000000000000000ffff0000000000000100000000000000fd87d87eeb4364f22cf54dca59412db7208d47d920cffce83ee8102f5361746f7368693a302e392e39392f2c9f040001").unwrap();
//
//    let decode: Result<VersionMessage, _> = deserialize(&from_sat);
//    println!("The version is {:#?}", decode.unwrap());

//    assert!(decode.is_ok());
//    let real_decode = decode.unwrap();
//    assert_eq!(real_decode.version, 70002);
//    assert_eq!(real_decode.services, 1);
//    assert_eq!(real_decode.timestamp, 1401217254);
//    // address decodes should be covered by Address tests
//    assert_eq!(real_decode.nonce, 16735069437859780935);
//    assert_eq!(real_decode.user_agent, "/Satoshi:0.9.99/".to_string());
//    assert_eq!(real_decode.start_height, 302892);
//    assert_eq!(real_decode.relay, true);
//
//    assert_eq!(serialize(&real_decode), from_sat);

    //现在开始组合消息 从example的send开始组合
    // 第一个是command s这个已经实现了 目标是编码成16进制的数组 形式和bitcoin Network protocol wiki上面表示的那样就行
    // 先构建struct 然后序列化
//    let cs = CommandString("version".to_owned());
//    let s= serialize(&cs);
//    println!("The s is {:0x?}", s);

    // 接下来是对payload 本身的编码 要发的第一个消息是Version 所以先对version编码
    // 步骤按照example中的来
    //
    // 先是version
    // addr_recv 也就是接受这个消息的ip地址定位 192.168.1.7 port 8332
    // addr_from 发送这个消息的ip地址我本机的ip是 192.168.101.61 port 指定一个 8332
    //
    //
//    let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;
//    let remote = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 7)), 8333);
//
//    let version = VersionMessage {
//        version: 70001,
//        services: 0000000000000000,//spv only
//        timestamp,
//        receiver: Address::new(&remote, 1),
//        // sender is only dummy
//        sender: Address::new(&remote, 1),
//        nonce: 0000000000000000,//not used here
//        user_agent: "/Alvin Example:0.9.3/".to_string(),
//        start_height: 329107,
//        relay: true,
//    };
//    let serialized_version = serialize(&version);
//    println!("The serialized_version is {:02x?}", &serialized_version);
//    println!("The len of serialized_version is {:?}", &serialized_version.len());

    // 接下来构建checksum
    // example 里面有这句话
    // Checksum is first 4 bytes of SHA256(SHA256(<payload>))
    // [https://en.bitcoin.it/wiki/Protocol_documentation#version]
    //
    // 为了解决这个问题，我去定义Verack 算一下Verack 的checksum
    //
    // 经验证使用hmac_sha256 计算即可 得到的结果和示例是一样的
    // [https://en.bitcoin.it/wiki/Protocol_documentation#verack]
    // 注意checksum值
    //

//    let hash_m1 = hmac_sha256::Hash::hash("".as_bytes());
//    let hash_m2 = hmac_sha256::Hash::hash(&hash_m1);
//    println!("I got hash_m SHA256(SHA256(verack)[0..4] {:0x?}", &hash_m2[0..4]);

    // 现在要序列化一下长度 很简单直接序列化x
    let length = serialize(&12);
    println!("The length is {:02x?}", length);

    // filterload 用固定值
    // 参考message/filterload

    // getdata数据得序列化一下 发现基本上已经实现了 不用自己动手
}

use hex::decode as hex_decode;
use crate::message::address::Address;
use std::net::{SocketAddr, IpAddr, Ipv4Addr, TcpStream};
use crate::message::version::VersionMessage;
use crate::message::command::CommandString;
use crate::message::{RawMessage, Payload, Magic};
use crate::message::filterload::FilterLoad;
use crate::message::getdata::GetData;
use std::io::{Write, Read};
use std::thread::sleep;
use std::time;
use log::info;

const IO_BUFFER_SIZE: usize = 1024 * 1024;

fn main(){
    let remote = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(192, 168, 1, 7)), 8333);
    //组装一个version
    let version_message = VersionMessage {
        version: 70001,
        services: 0,
        timestamp: 1415484102,
        receiver: Address::new(&remote, 1),
        // sender is only dummy
        sender: Address::new(&remote, 1),
        nonce: 0,
        bytes: 0x1b,
        user_agent: "/Bitcoin.org Example:0.9.3/".to_string(),
        start_height: 329107,
        relay: true,
    };

    let raw_version = RawMessage::new(Magic::Main,
                                      CommandString("version".to_owned()),
                                      Payload::Version(version_message),
    );
    let vec_version = raw_version.combine();
    info!("vec_version {:02x?}", &vec_version);


    //组装一个Verack Verack没有值
    let raw_verack = RawMessage::new(Magic::Main,
                                     CommandString("verack".to_owned()),
                                     Payload::Verack);
    let vec_verack = raw_verack.combine();
    info!("vec_verack {:02x?}", &vec_verack);

    //组装一个filterload

    //因为没有搞清楚机理  自己根据固定数据拼装一个filterload
    //send("filterload",
    //"02"  # ........ Filter bytes: 2
    //+ "b50f" # ....... Filter: 1010 1101 1111 0000
    //+ "0b000000" # ... nHashFuncs: 11
    //+ "00000000" # ... nTweak: 0/none
    //+ "00" # ......... nFlags: BLOOM_UPDATE_NONE
    //)
    let filterload_hex = "02b50f0b0000000000000000";
    let filterload_vec = hex_decode(filterload_hex).unwrap();
    info!("filterload_vec {:?}", filterload_vec);
    let raw_filterload = RawMessage::new(Magic::Main,
                                         CommandString("filterload".to_owned()),
                                         Payload::FilterLoad(FilterLoad(filterload_vec)));
    let vec_filterload = raw_filterload.combine();
    info!("vec_verack {:02x?}", &vec_filterload);

    //组装一个getdata 为了快速实现 也采用上述方法
    //send("getdata",
    //     "01" # ................................. Number of inventories: 1
    //    + "03000000" # ........................... Inventory type: filtered block
    //    + "a4deb66c0d726b0aefb03ed51be407fb"
    //   + "ad7331c6e8f9eef231b7000000000000" # ... Block header hash
    //)
    let getdata_hex = "0103000000a4deb66c0d726b0aefb03ed51be407fbad7331c6e8f9eef231b7000000000000";
    let getdata_vec = hex_decode(getdata_hex).unwrap();
    info!("getdata_vec {:?}", getdata_vec);
    let raw_getdata = RawMessage::new(Magic::Main,
                                      CommandString("getdata".to_owned()),
                                      Payload::GetData(GetData(getdata_vec)));
    let vec_getdata = raw_getdata.combine();
    info!("vec_verack {:02x?}", &vec_getdata);

    match TcpStream::connect("192.168.1.7:8333") {
        Ok(mut stream) => {
            info!("Successfully connected to server in port 8333");

            stream.write(&vec_version).unwrap();
            info!("Sent version, awaiting reply...");

            sleep(time::Duration::from_secs(1));

            stream.write(&vec_verack).unwrap();
            info!("Sent vec_verack, awaiting reply...");

            stream.write(&vec_filterload).unwrap();
            info!("Sent vec_filterload, awaiting reply...");

            stream.write(&vec_getdata).unwrap();
            info!("Sent vec_getdata, awaiting reply...");

            let mut iobuf = vec!(0u8; IO_BUFFER_SIZE);
            match stream.read(iobuf.as_mut_slice()) {
                Ok(len) => {
                    info!("{}", &len);
                    info!("{:02x?}", iobuf[0..len].to_vec())
                }
                Err(e) => {
                    info!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            info!("Failed to connect: {}", e);
        }
    }
    info!("Terminated.");
}