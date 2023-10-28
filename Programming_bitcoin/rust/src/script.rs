
use std::io::Read;

use num_bigint::BigInt;

use crate::utils;

#[derive(Debug,Clone)]
pub struct Script {
    pub cmds:Vec<Vec<u8>>,
}

impl Script {
    pub fn new(cmds:Option<Vec<Vec<u8>>>)->Self{
        match cmds {
            None => Script{cmds:Vec::new()},
            Some(cmds)=>Script{cmds},
        }
    }
    pub fn parse<R:Read>(stream:&mut R)->Self {
        let length =utils::read_varint(stream);
        let mut cmds: Vec<Vec<u8>> = Vec::new();
        let mut count = 0_u32;
        while count < length.to_u32_digits().1.pop().unwrap_or(0) {
            let mut op_buffer = vec![0;1];
            stream.read_exact(&mut op_buffer).unwrap();
            count+=1;
            let current_byte = BigInt::from_signed_bytes_be(&op_buffer).to_u32_digits().1.pop().unwrap();
            if current_byte >=1 && current_byte <=75 {
                let mut temp = vec![0,current_byte.try_into().unwrap()];
                stream.read_exact(&mut temp).unwrap();
                cmds.push(temp);
                count+=current_byte;
            } else if current_byte ==76 {
                let mut temp=vec![0;1];
                stream.read_exact(&mut temp).unwrap();
                let data_length = utils::little_endian_to_int(&temp);
                let mut temp = vec![0;data_length.clone().try_into().unwrap()];
                stream.read_exact(&mut temp).unwrap();
                cmds.push(temp);
                count+=data_length.to_u32_digits().1.pop().unwrap()+1;
            } else if current_byte == 77 {
                let mut temp = vec![0;2];
                stream.read_exact(&mut temp).unwrap();
                let data_length = utils::little_endian_to_int(&temp);
                let mut temp = vec![0;data_length.clone().try_into().unwrap()];
                stream.read_exact(&mut temp).unwrap();
                cmds.push(temp);
                count+=data_length.to_u32_digits().1.pop().unwrap()+2;
            } else {
                cmds.push(op_buffer);
            }
        }
        if count != length.to_u32_digits().1.pop().unwrap() {
            panic!("Parsing script failed");
        }

        Script { cmds }
    }

    fn raw_serialize(&self) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        for cmd in &self.cmds {
            if cmd.len() == 1 {
                result.push(cmd[0]);
            } else {
                let length = cmd.len();
                if length < 75 {
                    result.append(&mut  utils::usize_to_little_endian(length, 1))
                } else if length > 75 && length < 0x100 {
                    result.append(&mut utils::usize_to_little_endian(76, 1));
                    result.append(&mut utils::usize_to_little_endian(length, 1))
                } else if length >=0x100 && length <=520 {
                    result.append(&mut utils::usize_to_little_endian(77, 1));
                    result.append(&mut utils::usize_to_little_endian(length, 2));
                } else {
                    panic!("too long an cmd");
                }
                result.append(&mut cmd.clone())
            }
        }
        result
    }

    pub fn serialize(&self)->Vec<u8> {
        let result = self.raw_serialize();
        let total = result.len();
        [utils::encode_varint(total),result].concat()
    }

}


#[cfg(test)]
mod script_tests {
    use std::io::Cursor;

    use super::Script;

    #[test]
    fn test_parse_script() {
        let s = hex::decode("6a47304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a7160121035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937").unwrap();
        println!("{:?}",s);
        let mut cursor = Cursor::new(s);
        let s = Script::parse(&mut cursor);
        // println!("{:?}",hex::encode(s.cmds[1].clone()));
        // assert_eq!(hex::encode(s.cmds[0].clone()), "304402207899531a52d59a6de200179928ca900254a36b8dff8bb75f5f5d71b1cdc26125022008b422690b8461cb52c3cc30330b23d574351872b7c361e9aae3649071c1a71601");
        // assert_eq!(
        //     hex::encode(s.cmds[1].clone()),
        //     "035d5c93d9ac96881f19ba1f686f15f009ded7c62efe85a872e6a19b43c15a2937"
        // )
    }
}