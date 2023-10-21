use std::{fmt, ops::Add};

#[derive(Debug, Clone, Copy)]
struct Point2 {
    x: i32,
    y: i32,
    a: i32,
    b: i32,
}

#[derive(Debug, Clone, Copy)]
enum Point {
    Inf,
    Point(Point2),
}

impl Point {
    pub fn new(x: Option<i32>, y: Option<i32>, a: i32, b: i32) -> Point {
        if x.is_none() && y.is_none() {
            return Point::Inf;
        }

        match x {
            Some(x_l) => match y {
                Some(y_l) => {
                    if y_l.pow(2) != (x_l.pow(3) + a * x_l + b) {
                        panic!("({:?},{:?}) is not on the curve", x, y);
                    }
                    let a = Point2 {
                        x: x_l,
                        y: y_l,
                        a,
                        b,
                    };
                    Point::Point(a)
                }
                None => {
                    panic!("Invalid Inf Point")
                }
            },
            None => {
                panic!("Invalid Inf Point")
            }
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Point::Inf => match other {
                Point::Inf => true,
                Point::Point(Point2 {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                }) => false,
            },
            Point::Point(Point2 {
                x: x1,
                y: y1,
                a: a1,
                b: b1,
            }) => match other {
                Point::Inf => false,
                Point::Point(Point2 {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                }) => x1 == x2 && y1 == y2 && a1 == a2 && b1 == b2,
            },
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !(self == other)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Point::Inf => {
                write!(f, "Inf")
            }
            Point::Point(Point2 {
                x: x1,
                y: y1,
                a: _a1,
                b: _b1,
            }) => {
                write!(f, "({:?},{:?})", x1, y1)
            }
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        match self {
            Point::Inf => return other,
            Point::Point(Point2 {
                x: x1,
                y: y1,
                a: a1,
                b: b1,
            }) => match other {
                Point::Inf => self,
                Point::Point(Point2 {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                }) => {
                    if x1 == x2 && y1 != y2 {
                        return Point::Inf;
                    } else if x1 != x2 {
                        let s = (y2 - y1) / (x2 - x1);
                        let x3 = s.pow(2) - x1 - x2;
                        let y3 = s * (x1 - x3) - y1;
                        let p = Point2 {
                            x: x3,
                            y: y3,
                            a: a1,
                            b: b1,
                        };
                        return Point::Point(p);
                    } else {
                        let s = (3 * x1.pow(2) + a1) / (2 * y1);
                        let x3 = s.pow(3) - 2 * x1;
                        let y3 = s * (x1 - x3) - y1;
                        let p = Point2 {
                            x: x3,
                            y: y3,
                            a: a1,
                            b: b1,
                        };
                        return Point::Point(p);
                    }
                }
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq() {
        let p1 = Point::new(Some(-1), Some(-1), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        assert_eq!(p1, p2);
    }
    #[test]
    #[should_panic]
    fn test_ne() {
        let p1 = Point::new(Some(-1), Some(-1), 5, 7);
        let p2 = Point::new(Some(-1), Some(-2), 5, 7);
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_add() {
        let p1 = Point::new(Some(2), Some(5), 5, 7);
        let p2 = Point::new(Some(-1), Some(-1), 5, 7);
        let p3 = p1 + p2;
        print!("{:?}", p3);
    }
}
