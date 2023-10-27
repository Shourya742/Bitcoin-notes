use num_bigint::BigInt;
use num_integer::Integer;

const BASE58_ALPHABET: &'static [u8] =
    b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub fn encode_base58(s: &[u8]) -> String {
    let leading_zeros = s.iter().take_while(|&&x| x == 0).count();
    let mut num = BigInt::from_bytes_be(num_bigint::Sign::Plus, s);
    let mut result = String::new();
    let mut prefix = String::new();
    for _ in 0..leading_zeros {
        prefix.push('1');
    }
    while num > BigInt::from(0) {
        let (l_num, l_mod) = num.div_rem(&BigInt::from(58));
        (num) = l_num;
        let (sign, mut mod1) = (l_mod).to_u32_digits();
        let modd = match sign {
            num_bigint::Sign::Minus => panic!("Should not be negative"),
            num_bigint::Sign::NoSign => 0,
            num_bigint::Sign::Plus => mod1.pop().unwrap(),
        };
        let ch = BASE58_ALPHABET[modd as usize];
        result.push(ch as char);
    }
    prefix + (&result.chars().rev().collect::<String>()[..])
}

#[cfg(test)]
mod utils_tests {
    use super::encode_base58;

    #[test]
    fn base58_test() {
        let hex = &hex::decode("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d")
            .unwrap()[..];
        let base58 = encode_base58(hex);
        assert_eq!(base58, "9MA8fRQrT4u8Zj8ZRd6MAiiyaxb2Y1CMpvVkHQu5hVM6");
        let hex = &hex::decode("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c")
            .unwrap()[..];
        let base58 = encode_base58(hex);
        assert_eq!(base58, "4fE3H2E6XMp4SsxtwinF7w9a34ooUrwWe4WsW1458Pd");
        let hex = &hex::decode("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6")
            .unwrap()[..];
        let base58 = encode_base58(hex);
        assert_eq!(base58, "EQJsjkd6JaGwxrjEhfeqPenqHwrBmPQZjJGNSCHBkcF7");
        let hex = &b"\0\0\0\0abc"[..];
        let base58 = encode_base58(hex);
        assert_eq!(base58, "1111ZiCa");
    }
}
