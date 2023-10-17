use std::{
    fmt,
    ops::{Add, Div, Mul, Sub},
};

#[derive(Debug, Clone, Eq)]
pub struct FiniteField {
    pub num: isize,
    pub prime: isize,
}

impl FiniteField {
    pub fn new(num: isize, prime: isize) -> Self {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime)
        }

        FiniteField { num, prime }
    }

    pub fn pow(self, exponent: isize) -> Self {
        let mut new_num = 1;
        let new_exp = (exponent % (self.prime - 1) + self.prime - 1) % (self.prime - 1);

        for _ in 0..new_exp {
            new_num = (new_num * self.num) % self.prime;
        }

        FiniteField {
            num: new_num,
            prime: self.prime,
        }
    }
}
impl PartialEq for FiniteField {
    fn eq(&self, other: &Self) -> bool {
        self.num == other.num && self.prime == other.prime
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl fmt::Display for FiniteField {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl Add for FiniteField {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot add two numbers with different fields")
        }
        let new_num = (self.num + other.num) % self.prime;
        FiniteField {
            num: new_num,
            prime: self.prime,
        }
    }
}

impl Sub for FiniteField {
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot substract two number with different fields")
        }
        let new_num = (self.num - other.num + self.prime) % self.prime;
        FiniteField {
            num: new_num,
            prime: self.prime,
        }
    }
}

impl Mul for FiniteField {
    type Output = Self;
    fn mul(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot multiply two numbers with different Field")
        }
        let new_num = (self.num * other.num) % self.prime;
        FiniteField {
            num: new_num,
            prime: self.prime,
        }
    }
}

impl Div for FiniteField {
    type Output = Self;
    fn div(self, other: Self) -> Self::Output {
        if self.prime != other.prime {
            panic!("Cannot Divide two numbers with different field")
        }
        let exp: isize = self.prime - 2;
        let power_var = other.pow(exp);
        return self * power_var;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_eq() {
        let a = FiniteField::new(7, 13);
        let b = FiniteField::new(6, 13);
        assert_ne!(a, b);
        assert_eq!(a, a);
    }

    #[test]
    fn test_add() {
        let a = FiniteField::new(7, 13);
        let b = FiniteField::new(12, 13);
        let c = FiniteField::new(6, 13);
        assert_eq!(a + b, c);
    }

    #[test]
    fn test_sub() {
        let a = FiniteField::new(2, 13);
        let b = FiniteField::new(11, 13);
        let c = FiniteField::new(9, 13);
        assert_eq!(b - c, a);
    }

    #[test]
    fn test_mul() {
        let a = FiniteField::new(3, 13);
        let b = FiniteField::new(12, 13);
        let c = FiniteField::new(4, 13);
        assert_eq!(a * c, b);
        let a = FiniteField::new(3, 13);
        let b = FiniteField::new(5, 13);
        let c = FiniteField::new(2, 13);
        assert_eq!(a * b, c);
    }
    #[test]
    fn test_pow() {
        let a = FiniteField::new(3, 13);
        let b = a.pow(2);
        let c = FiniteField::new(9, 13);
        assert_eq!(b, c);
        let a = FiniteField::new(3, 13);
        let b = a.pow(-4);
        let c = FiniteField { num: 9, prime: 13 };
        assert_eq!(b, c);
    }

    #[test]
    fn test_div() {
        let a = FiniteField::new(2, 13);
        let b = FiniteField::new(7, 13);
        let c = a / b;
        assert_eq!(c, FiniteField::new(4, 13));
    }
}
