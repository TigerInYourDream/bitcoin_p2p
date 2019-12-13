//send("getdata",
//     "01" # ................................. Number of inventories: 1
//    + "03000000" # ........................... Inventory type: filtered block
//    + "a4deb66c0d726b0aefb03ed51be407fb"
//   + "ad7331c6e8f9eef231b7000000000000" # ... Block header hash
//)

pub struct GetData(pub Vec<u8>);