use std::{clone, fmt, ops::Add};

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
}
