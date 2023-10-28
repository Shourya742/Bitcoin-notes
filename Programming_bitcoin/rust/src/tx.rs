use std::io::{Read, BufReader};

use num_bigint::BigInt;
use crate::utils;

#[derive(Debug)]
pub struct Tx{
    version:BigInt,
    tx_ins:Option<Vec<i32>>,
    tx_outs:Option<Vec<i32>>,
    locktime:Option<i32>,
    testnet:bool
}

impl Tx {
    pub fn new(version:i32,tx_ins:Vec<i32>,tx_outs:Vec<i32>,locktime:i32,testnet:bool)->Self{
        todo!()
    }

    pub fn parse<R:Read>(stream:BufReader<R>,testnet:bool)->Self {
        let mut buffer = [0;4];
        let mut socket = stream.take(4);
        socket.read(&mut buffer).unwrap();
        let version = utils::little_endian_to_int(&buffer);
        println!("Version {:?}",version);
        Tx { version, tx_ins: None, tx_outs: None, locktime: None, testnet }

    }
}