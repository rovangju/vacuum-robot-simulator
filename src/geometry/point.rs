use std::cmp;
use std::fmt;

use super::Vector;
use math::Scalar;

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub pos: Vector,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Point({}, {})", self.pos.x, self.pos.y)
    }
}

impl cmp::PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.pos == other.pos
    }
}

impl Point {
    pub fn new(x: Scalar, y: Scalar) -> Point {
        Point {
            pos: Vector::new(x, y),
        }
    }

    pub fn from_vector(pos: Vector) -> Point {
        Point { pos }
    }

    pub fn distance(&self) -> Scalar {
        self.pos.length()
    }
}
