use std::{
    clone, fmt,
    ops::{Add, Mul},
    rc::Rc,
};

use num_bigint::BigInt;

use crate::{finite_field::FiniteElement, PointWrapper};

impl PointWrapper<FiniteElement> {
    pub fn new(x: FiniteElement, y: FiniteElement, a: FiniteElement, b: FiniteElement) -> Self {
        if y.pow(BigInt::from(2)) != (x.pow(BigInt::from(3)) + a.clone() * x.clone() + b.clone()) {
            panic!("({:?}, {:?}) is not on the curve", x, y);
        }
        PointWrapper::Point { x, y, a, b }
    }

    pub fn new_inf() -> Self {
        PointWrapper::Inf
    }
}

impl PartialEq for PointWrapper<FiniteElement> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PointWrapper::Inf, PointWrapper::Inf) => true,
            (PointWrapper::Inf, PointWrapper::Point { .. }) => false,
            (PointWrapper::Point { .. }, PointWrapper::Inf) => false,
            (
                PointWrapper::Point {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                PointWrapper::Point {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                },
            ) => x1 == x2 && y1 == y2 && a1 == a2 && b1 == b2,
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl fmt::Display for PointWrapper<FiniteElement> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PointWrapper::Inf => {
                write!(f, "infinity")
            }
            PointWrapper::Point {
                x: x1,
                y: y1,
                a: a1,
                b: b1,
            } => {
                write!(
                    f,
                    "Point({:?},{:?})_{}_{} FieldElement({})",
                    x1.num, y1.num, a1.num, b1.num, x1.prime
                )
            }
        }
    }
}

impl Add for PointWrapper<FiniteElement> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (PointWrapper::Inf, PointWrapper::Inf) => PointWrapper::Inf,
            (PointWrapper::Inf, p @ PointWrapper::Point { .. }) => p.clone(),
            (p @ PointWrapper::Point { .. }, PointWrapper::Inf) => p.clone(),
            (
                PointWrapper::Point {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                PointWrapper::Point {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                },
            ) => {
                let p1 = PointWrapper::Point {
                    x: x1.clone(),
                    y: y1.clone(),
                    a: a1.clone(),
                    b: b1.clone(),
                };
                let p2 = PointWrapper::Point {
                    x: x2.clone(),
                    y: y2.clone(),
                    a: a2.clone(),
                    b: b2.clone(),
                };
                if a1 != a2 || b1 != b2 {
                    panic!("Points {},{} are not on the same curve", p1, p2);
                }

                if x1 == x2 && y1 != y2 {
                    return PointWrapper::Inf;
                } else if x1 != x2 {
                    let s = (y2.clone() - y1.clone()) / (x2.clone() - x1.clone());
                    let x = s.pow(BigInt::from(2)) - x1.clone() - x2.clone();
                    let y = s * (x1.clone() - x.clone()) - y1.clone();
                    return PointWrapper::Point {
                        x,
                        y,
                        a: a1.clone(),
                        b: b1.clone(),
                    };
                } else if p1 == p2
                    && y1.clone()
                        == FiniteElement::new_big_int(BigInt::from(0), x1.clone().prime).unwrap()
                {
                    PointWrapper::Inf
                } else if p1 == p2 {
                    let s = (FiniteElement::new_big_int(BigInt::from(3), x1.clone().prime)
                        .unwrap()
                        * x1.clone().pow(BigInt::from(2))
                        + a1.clone())
                        / (FiniteElement::new_big_int(BigInt::from(2), x1.clone().prime).unwrap()
                            * y1.clone());
                    let x = s.pow(BigInt::from(2))
                        - FiniteElement::new_big_int(BigInt::from(2), x1.clone().prime).unwrap()
                            * x1.clone();
                    let y = s * (x1.clone() - x.clone()) - y1.clone();
                    return PointWrapper::Point {
                        x,
                        y,
                        a: a1.clone(),
                        b: b1.clone(),
                    };
                } else {
                    panic!("no more cases")
                }
            }
        }
    }
}

impl Mul<PointWrapper<FiniteElement>> for i32 {
    type Output = PointWrapper<FiniteElement>;
    fn mul(self, rhs: PointWrapper<FiniteElement>) -> Self::Output {
        let mut coef = self;
        let mut current = rhs;
        let mut result = PointWrapper::new_inf();
        while coef > 0 {
            if coef & 1 == 1 {
                result = result + current.clone();
            }
            current = current.clone() + current;
            coef >>= 1;
        }
        result
    }
}

impl Mul<PointWrapper<FiniteElement>> for BigInt {
    type Output = PointWrapper<FiniteElement>;
    fn mul(self, rhs: PointWrapper<FiniteElement>) -> Self::Output {
        let mut coef = Rc::new(self);
        let mut current = rhs;
        let mut result = PointWrapper::new_inf();
        while coef.as_ref() > &BigInt::from(0) {
            if coef.as_ref() & BigInt::from(1) == BigInt::from(1) {
                result = result + current.clone();
            }
            current = current.clone() + current;
            *Rc::get_mut(&mut coef).unwrap() >>= 1;
        }
        result
    }
}

#[cfg(test)]
mod point_finite_field_test {
    use crate::{finite_field::FiniteElement, PointWrapper};
    struct TestPoint {
        p1: (i32, i32),
        p2: (i32, i32),
        res: (i32, i32),
    }
    #[test]
    fn test_on_curve() -> Result<(), String> {
        let prime = 223;
        let a = FiniteElement::new(0, prime).unwrap();
        let b = FiniteElement::new(7, prime).unwrap();
        let valid_points = vec![(192, 105), (17, 56), (1, 193)];
        for (x_raw, y_raw) in valid_points {
            let x = FiniteElement::new(x_raw, prime).unwrap();
            let y = FiniteElement::new(y_raw, prime).unwrap();
            PointWrapper::new(x, y, a.clone(), b.clone());
        }
        Ok(())
    }

    #[test]
    fn test_mul_binary_expansion() {
        let prime = 223;
        let a = FiniteElement::new(0, prime).unwrap();
        let b = FiniteElement::new(7, prime).unwrap();
        let p5 = PointWrapper::new(
            FiniteElement::new(15, prime).unwrap(),
            FiniteElement::new(86, prime).unwrap(),
            a.clone(),
            b.clone(),
        );
        assert_eq!(PointWrapper::new_inf(), 7 * p5);

        let p5 = PointWrapper::new(
            FiniteElement::new(47, prime).unwrap(),
            FiniteElement::new(71, prime).unwrap(),
            a.clone(),
            b.clone(),
        );
        let res8 = PointWrapper::new(
            FiniteElement::new(116, prime).unwrap(),
            FiniteElement::new(55, prime).unwrap(),
            a.clone(),
            b.clone(),
        );

        let res4 = PointWrapper::new(
            FiniteElement::new(194, prime).unwrap(),
            FiniteElement::new(51, prime).unwrap(),
            a.clone(),
            b.clone(),
        );

        let res2 = PointWrapper::new(
            FiniteElement::new(36, prime).unwrap(),
            FiniteElement::new(111, prime).unwrap(),
            a.clone(),
            b.clone(),
        );
        assert_eq!(res2, 2 * p5.clone());
        assert_eq!(res4, 4 * p5.clone());
        assert_eq!(res8, 8 * p5.clone());
    }
}
