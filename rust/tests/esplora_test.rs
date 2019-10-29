//extern crate reqwest;
//extern crate futures;
//extern crate tokio;
//extern crate serde;
//extern crate serde_json;
//
//use serde::{Deserialize, Serialize};
//use std::mem;
//use std::io::{self, Cursor};
//use futures::{Future, Stream};
//use reqwest::r#async::{Client, Decoder};
//
///**
//{
//  "address": "2MsM67NLa71fHvTUBqNENW15P68nHB2vVXb",
//  "chain_stats": {
//    "funded_txo_count": 5612,
//    "funded_txo_sum": 239991076688,
//    "spent_txo_count": 0,
//    "spent_txo_sum": 0,
//    "tx_count": 5612
//  },
//  "mempool_stats": {
//    "funded_txo_count": 0,
//    "funded_txo_sum": 0,
//    "spent_txo_count": 0,
//    "spent_txo_sum": 0,
//    "tx_count": 0
//  }
//}
//**/
//#[derive(Debug, Serialize, Deserialize)]
//struct Address {
//    address: String,
//    chain_stats: AddressStats,
//    mempool_stats: AddressStats,
//}
//
//#[derive(Debug, Serialize, Deserialize)]
//struct AddressStats {
//    funded_txo_count: u32,
//    funded_txo_sum: u64,
//    spent_txo_count: u32,
//    spent_txo_sum: u64,
//    tx_count: u32
//}
//
//#[test]
//fn test1() -> Result<(), reqwest::Error> {
//    let address: Address = reqwest::Client::new()
//        .get("https://blockstream.info/testnet/api/address/2MsM67NLa71fHvTUBqNENW15P68nHB2vVXb")
//        .send()?
//        .json()?;
//    println!("{:#?}", address);
//    Ok(())
//}
//
//fn fetch() -> impl Future<Item=(), Error=()> {
//    Client::new()
//        .get("https://hyper.rs")
//        .send()
//        .and_then(|mut res| {
//            println!("{}", res.status());
//
//            let body = mem::replace(res.body_mut(), Decoder::empty());
//            body.concat2()
//        })
//        .map_err(|err| println!("request error: {}", err))
//        .map(|body| {
//            let mut body = Cursor::new(body);
//            let _ = io::copy(&mut body, &mut io::stdout())
//                .map_err(|err| {
//                    println!("stdout error: {}", err);
//                });
//        })
//}
//
//#[ignore]
//#[test]
//fn test2() {
//    tokio::run(fetch());
//}