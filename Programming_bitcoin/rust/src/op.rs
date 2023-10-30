use std::io::Cursor;
use byteorder::{BigEndian,ByteOrder};
use num_bigint::BigInt;
use crate::{s256_point::S256Point,signature::Signature,utils};

#[derive(Debug,Clone)]
pub enum OpCodeFunctions {
    Op0(u32),
    OpChecksig(u32),
    OpDup(u32),
    OpHash160(u32),
    OpHash256(u32),
    OpEqualverify(u32),
    OpEqual(u32)
}

impl AsRef<u32> for OpCodeFunctions {
    fn as_ref(&self) -> &u32 {
        match &self {
            OpCodeFunctions::Op0(op)=>op,
            OpCodeFunctions::OpChecksig(op)=>op,
            OpCodeFunctions::OpDup(op)=>op,
            OpCodeFunctions::OpEqual(op)=>op,
            OpCodeFunctions::OpEqualverify(op)=>op,
            OpCodeFunctions::OpHash256(op)=>op,
            OpCodeFunctions::OpHash160(op)=>op
        }
    }
}

pub fn parse_op_codes(op_code:u32)->OpCodeFunctions {
    match op_code {
        0=>OpCodeFunctions::Op0(op_code),
        172=>OpCodeFunctions::OpChecksig(op_code),
        118=>OpCodeFunctions::OpDup(op_code),
        169=>OpCodeFunctions::OpHash160(op_code),
        136=>OpCodeFunctions::OpEqualverify(op_code),
        135=>OpCodeFunctions::OpEqual(op_code),
        170=>OpCodeFunctions::OpHash256(op_code),
        _=>panic!("Unknown opCode"),
    }
}


fn decode_num(element:Vec<u8>)->i32 {
    if element == b"".to_vec() {
        return 0;
    }
    let mut result:i32;
    let mut big_endian = element.clone();
    big_endian.reverse();
    let negative:bool;
    if(big_endian[0] & 0x80) == 1 {
        negative = true;
        result = BigEndian::read_u32(&[0,0,0,big_endian[0] & 0x7f]) as i32;
    } else {
        negative = false;
        result = BigEndian::read_i32(&[0,0,0,big_endian[0]]) as i32;
    }
    for c in &big_endian[1..] {
        result <<=8;
        result+=BigEndian::read_i32(&[0,0,0,c.clone()]) as i32;
    }

    if negative {
        return -result;
    } else {
        return result;
    }
}