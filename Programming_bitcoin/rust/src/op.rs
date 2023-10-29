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