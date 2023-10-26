pub mod finite_field;
pub mod finite_field_point;
pub mod private_key;
pub mod real_numbers_point;
pub mod signature;
use finite_field::FiniteElement;
use num_bigint::BigInt;
use once_cell::sync::Lazy;
use signature::Signature;
use std::{fmt::Display, ops::Mul};
#[derive(Debug, Clone, Copy)]
pub enum PointWrapper<A> {
    Inf,
    Point { x: A, y: A, a: A, b: A },
}

#[derive(Debug, Clone)]
pub struct S256Point {
    point: PointWrapper<FiniteElement>,
}

#[derive(Debug, Clone)]
pub struct S256Field {
    field: FiniteElement,
}

impl S256Field {
    pub fn new(num: BigInt) -> S256Field {
        let p: BigInt = BigInt::from(2).pow(256) - BigInt::from(2).pow(32) - BigInt::from(977);
        S256Field {
            field: FiniteElement::new_big_int(num, p).unwrap(),
        }
    }
}

impl Display for S256Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#064x}", self.field.num)
    }
}

pub static N: Lazy<BigInt> = Lazy::new(|| {
    BigInt::parse_bytes(
        b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
        16,
    )
    .unwrap()
});

pub static G: Lazy<S256Point> = Lazy::new(|| {
    let x = BigInt::parse_bytes(
        b"79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798",
        16,
    )
    .unwrap();
    let y = BigInt::parse_bytes(
        b"483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8",
        16,
    )
    .unwrap();
    S256Point::new(S256Field::new(x), S256Field::new(y))
});

impl S256Point {
    pub fn new(x: S256Field, y: S256Field) -> S256Point {
        let a = FiniteElement::new_big_int(BigInt::from(0), x.field.clone().prime).unwrap();
        let b = FiniteElement::new_big_int(BigInt::from(7), x.field.clone().prime).unwrap();
        S256Point {
            point: PointWrapper::new(x.field, y.field, a, b),
        }
    }
    pub fn verify(self, z: BigInt, sig: Signature) -> bool {
        let n_2 = N.to_owned() - 2;
        let s_inv = sig.s.modpow(&n_2, &N);
        let u = (z * s_inv.clone()).modpow(&BigInt::from(1), &N);
        let v = (sig.r.clone() * s_inv.clone()).modpow(&BigInt::from(1), &N);
        let total = u * G.to_owned() + v * self;
        match total {
            PointWrapper::Inf => false,
            PointWrapper::Point {
                x,
                y: _,
                a: _,
                b: _,
            } => x.num == sig.r,
        }
    }

    pub fn sec(self) -> String {
        match self.point {
            PointWrapper::Inf => panic!("Public Key point cannot be point to infinity"),
            PointWrapper::Point { x, y, a: _, b: _ } => {
                let marker = &b"\x04"[0..1];
                let x = &x.num.to_bytes_be().1.to_vec()[0..32];
                let y = &y.num.to_bytes_be().1.to_vec()[0..32];
                let res = [marker, x, y];
                let res = res.concat();
                hex::encode(&res)
            }
        }
    }
}

impl Mul<S256Point> for BigInt {
    type Output = PointWrapper<FiniteElement>;
    fn mul(self, rhs: S256Point) -> Self::Output {
        let n = BigInt::parse_bytes(
            b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        )
        .unwrap();
        let coef = self.modpow(&BigInt::from(1), &n);
        coef * rhs.point
    }
}

#[cfg(test)]
mod secp256k1_test {
    use num_bigint::BigInt;

    use crate::{
        private_key::PrivateKey, signature::Signature, PointWrapper, S256Field, S256Point, G, N,
    };
    use hex_literal::hex;
    use sha2::{Digest, Sha256};

    #[test]
    fn s256_point_test() {
        assert_eq!(PointWrapper::new_inf(), N.to_owned() * G.point.clone());
    }

    #[test]
    fn point_verification_test() {
        let z: BigInt = BigInt::parse_bytes(
            b"bc62d4b80d9e36da29c16c5d4d9f11731f36052c72401a76c23c0fb5a9b74423",
            16,
        )
        .unwrap();
        let r: BigInt = BigInt::parse_bytes(
            b"37206a0610995c58074999cb9767b87af4c4978db68c06e8e6e81d282047a7c6",
            16,
        )
        .unwrap();
        let s: BigInt = BigInt::parse_bytes(
            b"8ca63759c1157ebeaec0d03cecca119fc9a75bf8e6d0fa65c841c8e2738cdaec",
            16,
        )
        .unwrap();

        let px: BigInt = BigInt::parse_bytes(
            b"04519fac3d910ca7e7138f7013706f619fa8f033e6ec6e09370ea38cee6a7574",
            16,
        )
        .unwrap();
        let py: BigInt = BigInt::parse_bytes(
            b"82b51eab8c27c66e26c858a079bcdf4f1ada34cec420cafc7eac1a42216fb6c4",
            16,
        )
        .unwrap();
        let point = S256Point::new(S256Field::new(px), S256Field::new(py));
        let sig = Signature::new(r, s);
        assert!(point.verify(z, sig))
    }

    #[test]
    fn point_sign_test() {
        let sha256_z = Sha256::digest(Sha256::digest(b"Programming Bitcoin in Rust!"));
        let e = BigInt::from(12345);
        let z = BigInt::from_bytes_be(num_bigint::Sign::Plus, &sha256_z);
        let p = PrivateKey::new(e);
        let (public_x, public_y) = match p.point.clone().point {
            PointWrapper::Inf => panic!("public key should not be point to infinity"),
            PointWrapper::Point { x, y, a: _, b: _ } => (x, y),
        };
        let sig = p.sign(z.clone(), Some(BigInt::from(1234567890)));
        assert_eq!(
            public_x.num.to_bytes_be().1,
            hex!("f01d6b9018ab421dd410404cb869072065522bf85734008f105cf385a023a80f")
        );
        assert_eq!(
            public_y.num.to_bytes_be().1,
            hex!("0eba29d0f0c5408ed681984dc525982abefccd9f7ff01dd26da4999cf3f6a295")
        );

        assert!(p.point.clone().verify(z, sig))
    }

    #[test]
    fn test_256point_sec() {
        assert_eq!(PrivateKey::new(BigInt::from(5000)).point.sec(),"04ffe558e388852f0120e46af2d1b370f85854a8eb0841811ece0e3e03d282d57c315dc72890a4f10a1481c031b03b351b0dc79901ca18a00cf009dbdb157a1d10");
        assert_eq!(PrivateKey::new(BigInt::from(2018).pow(5)).point.sec(), "04027f3da1918455e03c46f659266a1bb5204e959db7364d2f473bdf8f0a13cc9dff87647fd023c13b4a4994f17691895806e1b40b57f4fd22581a4f46851f3b06");
        assert_eq!(PrivateKey::new(BigInt::parse_bytes(b"deadbeef12345", 16).unwrap()).point.sec(), "04d90cd625ee87dd38656dd95cf79f65f60f7273b67d3096e68bd81e4f5342691f842efa762fd59961d0e99803c61edba8b3e3f7dc3a341836f97733aebf987121");
    }
}
