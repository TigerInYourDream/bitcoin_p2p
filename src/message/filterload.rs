//! filterload 暂时设定为固定值
//!
//send("filterload",
//"02"  # ........ Filter bytes: 2
//+ "b50f" # ....... Filter: 1010 1101 1111 0000
//+ "0b000000" # ... nHashFuncs: 11
//+ "00000000" # ... nTweak: 0/none
//+ "00" # ......... nFlags: BLOOM_UPDATE_NONE
//)

//pub struct FilterLoad {
//    pub bytes: u8,
//    pub filter: String,
//    pub nHashFuncs: u32,
//    pub nTweak: u32,
//    pub nFlags: u8,
//}
//
//impl_consensus_encoding!(FilterLoad,bytes,
//                         filter, nHashFuncs,
//                         nTweak, nFlags);

//先根据固定数组组装一个
pub struct FilterLoad(pub Vec<u8>);