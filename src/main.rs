//! use this project to send p2p_message to my own full_node.
//! because I didn't send right message via murmel.
//! I could not add to extern type in murmel's NetworkMessage enum, and i didn't get right value from bitcoin Network。
//! In this project I will use tokios not mio in murmel. And I will send message to bitcoin Testnet.
//! You can see the example in [https://bitcoin.org/en/developer-examples#retrieving-a-merkleblock].
//! The code is writen in python.
//!
//! Here is dependency library
//! bitcoin_hashes for hash [https://crates.io/crates/bitcoin_hashes]
//! tokios for tcp connections
//! rust_bitcoin for de/serialization, parsing and executing on data structures and network messages

pub mod message;

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

fn main() {
    simple_logger::init().unwrap();
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




