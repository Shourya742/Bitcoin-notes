pub mod finite_field;
pub mod finite_field_point;
pub mod real_numbers_point;
use finite_field::FiniteElement;
use num_bigint::BigInt;
use std::{fmt::Display, ops::Mul};
#[derive(Debug, Clone, Copy)]
pub enum PointWrapper<A> {
    Inf,
    Point { x: A, y: A, a: A, b: A },
}

pub struct S256Point {
    point: PointWrapper<FiniteElement>,
}

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
        write!(f, "{:x}", self.field.num)
    }
}

impl S256Point {
    pub fn new(x: S256Field, y: S256Field) -> S256Point {
        let a = FiniteElement::new_big_int(BigInt::from(0), x.field.clone().prime).unwrap();
        let b = FiniteElement::new_big_int(BigInt::from(7), x.field.clone().prime).unwrap();
        S256Point {
            point: PointWrapper::new(x.field, y.field, a, b),
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
    use crate::{PointWrapper, S256Field, S256Point};
    use num_bigint::BigInt;
    #[test]
    fn s256_point_test() {
        let n = BigInt::parse_bytes(
            b"fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141",
            16,
        )
        .unwrap();
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
        let g = S256Point::new(S256Field::new(x), S256Field::new(y));
        assert_eq!(PointWrapper::new_inf(), n * g);
    }
}
