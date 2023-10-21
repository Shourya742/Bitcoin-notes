use std::{fmt, ops::Add};

#[derive(Debug, Clone, Copy)]
enum Point {
    Inf,
    Point { x: i32, y: i32, a: i32, b: i32 },
}

impl Point {
    pub fn new(x: i32, y: i32, a: i32, b: i32) -> Point {
        if y.pow(2) != (x.pow(3) + a * x + b) {
            panic!("({:?},{:?}) is not on the curve", x, y);
        }

        Point::Point { x, y, a, b }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Point::Inf, Point::Inf) => true,
            (Point::Inf, Point::Point { .. }) => false,
            (Point::Point { .. }, Point::Inf) => false,
            (
                Point::Point {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                Point::Point {
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

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Point::Inf => {
                write!(f, "Infinity")
            }
            Point::Point {
                x: x1,
                y: y1,
                a: _a1,
                b: _b1,
            } => {
                write!(f, "Point({:?},{:?})_{}_{}", x1, y1, _a1, _b1)
            }
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Point::Inf, Point::Inf) => Point::Inf,
            (Point::Inf, p @ Point::Point { .. }) => p,
            (p @ Point::Point { .. }, Point::Inf) => p,
            (
                p1 @ Point::Point {
                    x: x1,
                    y: y1,
                    a: a1,
                    b: b1,
                },
                p2 @ Point::Point {
                    x: x2,
                    y: y2,
                    a: a2,
                    b: b2,
                },
            ) => {
                if a1 != a2 || b1 != b2 {
                    panic!("Points {},{} are not in the same curve", p1, p2)
                }
                if x1 == x2 && y1 != y2 {
                    return Point::Inf;
                } else if x1 != x2 {
                    let s = (y2 - y1) / (x2 - x1);
                    let x = s.pow(2) - x1 - x2;
                    let y = s * (x1 - x) - y1;
                    return Point::Point { x, y, a: a1, b: b1 };
                } else if p1 == p2 && y1 == 0 {
                    Point::Inf
                } else if p1 == p2 {
                    let s = (3 * x1.pow(2) + a1) / (2 * y1);
                    let x = s.pow(2) - 2 * x1;
                    let y = s * (x1 - x) - y1;
                    return Point::Point { x, y, a: a1, b: b1 };
                } else {
                    panic!("No more casses")
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq() {
        let p1 = Point::new(-1, -1, 5, 7);
        let p2 = Point::new(-1, -1, 5, 7);
        assert_eq!(p1, p2);
    }
    #[test]
    #[should_panic]
    fn test_ne() {
        let p1 = Point::new(-1, -1, 5, 7);
        let p2 = Point::new(-1, -2, 5, 7);
        assert_ne!(p1, p2);
    }

    #[test]
    fn test_add() {
        let p1 = Point::new(2, 5, 5, 7);
        let p2 = Point::new(-1, -1, 5, 7);
        let p3 = p1 + p2;
        print!("{:?}", p3);
    }
    #[test]
    fn add_inf() {
        let p1 = Point::new(-1, -1, 5, 7);
        let p2 = Point::new(-1, 1, 5, 7);
        let inf = Point::Inf;
        assert_eq!(p1 + inf, p1);
        assert_eq!(inf + p2, p2);
        assert_eq!(p1 + p2, inf);
    }
    #[test]
    fn add_different_x() {
        let p1 = Point::new(3, 7, 5, 7);
        let p2 = Point::new(-1, -1, 5, 7);
        let p3 = Point::new(2, -5, 5, 7);
        assert_eq!(p1 + p2, p3);
    }
    #[test]
    fn add_same_point() {
        let p = Point::new(-1, -1, 5, 7);
        let p2 = Point::new(18, 77, 5, 7);
        assert_eq!(p + p, p2);
    }
    #[test]
    fn add_same_x_different_y() {
        let p1 = Point::new(-1, 1, 5, 7);
        let p2 = Point::new(-1, -1, 5, 7);
        assert_eq!(p1 + p2, Point::Inf);
    }
}
