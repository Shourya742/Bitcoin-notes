use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

use anyhow::{bail, Result};
use num_bigint::BigInt;

#[derive(Debug, Clone, Eq)]
pub struct FiniteElement {
    pub num: BigInt,
    pub prime: BigInt,
}

impl FiniteElement {
    pub fn new(num: i32, prime: i32) -> Result<Self> {
        if num >= prime {
            bail!("Num {} not in field range 0 to {}", num, prime - 1)
        }
        Ok(FiniteElement {
            num: BigInt::from(num),
            prime: BigInt::from(prime),
        })
    }

    pub fn new_big_int(num: BigInt, prime: BigInt) -> Result<Self> {
        if num >= prime {
            bail!("Num {} not in field range 0 to {}", num, prime - 1);
        }
        Ok(FiniteElement { num, prime })
    }

    pub fn pow(&self, exponent: BigInt) -> Self {
        let positive_exponent =
            exponent.modpow(&BigInt::from(1), &(self.prime.clone() - BigInt::from(1)));
        let num = self.num.modpow(&positive_exponent, &self.prime);
        FiniteElement {
            num,
            prime: self.prime.clone(),
        }
    }
}

impl PartialEq for FiniteElement {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }
    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl fmt::Display for FiniteElement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl Add for FiniteElement {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields")
        }
        let num = (self.num + other.num).modpow(&BigInt::from(1), &self.prime);
        FiniteElement {
            num,
            prime: self.prime,
        }
    }
}

impl Sub for FiniteElement {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot sub two numbers in different Fields");
        }
        let num = (self.num - other.num).modpow(&BigInt::from(1), &self.prime);
        FiniteElement {
            num,
            prime: self.prime,
        }
    }
}

impl Mul for FiniteElement {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot mutliply two numbers in different Fields");
        }
        let num = (self.num * other.num).modpow(&BigInt::from(1), &self.prime);
        FiniteElement {
            num,
            prime: self.prime,
        }
    }
}

impl Div for FiniteElement {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers in different Fields")
        }
        let exp = self.prime.clone() - BigInt::from(2);
        let num = (self.num * other.num.modpow(&exp, &self.prime.clone()))
            .modpow(&BigInt::from(1), &self.prime);
        FiniteElement {
            num,
            prime: self.prime,
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_ne() {
        let a = FiniteElement::new(7, 13).unwrap();
        let b = FiniteElement::new(6, 13).unwrap();
        println!("{:?}", a);
        println!("{:?}", b);
        assert_ne!(a, b);
    }

    #[test]
    fn test_eq() {
        let a = FiniteElement::new(7, 13).unwrap();
        let b = FiniteElement::new(7, 13).unwrap();
        assert_eq!(a, b)
    }

    #[test]
    fn test_sum() {
        let a = FiniteElement::new(7, 13).unwrap();
        let b = FiniteElement::new(12, 13).unwrap();
        let c = FiniteElement::new(6, 13).unwrap();
        assert_eq!(a + b, c)
    }

    #[test]
    fn test_excersize5() {
        let a = vec![1, 3, 7, 13, 18];
        let mut b = Vec::new();
        for i in 0..19 {
            b.push(i);
        }
        for (index, value) in a.iter().enumerate() {
            println!("Processing for batch {} starts", index);
            for x in b.iter() {
                print!("{} ", (x * value) % 19)
            }
            println!("")
        }
    }

    #[test]
    fn test_mul() {
        let a = FiniteElement::new(3, 13).unwrap();
        let b = FiniteElement::new(12, 13).unwrap();
        let c = FiniteElement::new(10, 13).unwrap();
        assert_eq!(a * b, c)
    }

    #[test]
    fn test_neg_pow() {
        let a = FiniteElement::new(7, 13).unwrap();
        let b = FiniteElement::new(8, 13).unwrap();
        assert_eq!(a.pow(BigInt::from(-3)), b);
    }
}
