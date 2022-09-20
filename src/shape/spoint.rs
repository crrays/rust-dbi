use std::fmt;
use std::ops;

use super::svector::Vector;

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}


impl ops::Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Point {
        Point { 
            x: self.x + rhs.x, 
            y: self.y + rhs.y 
        }
    }
}

impl ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Point {
        Point { 
            x: self.x - rhs.x, 
            y: self.y - rhs.y 
        }
    }
}


impl Point {
    
    pub fn new(x:u32, y:u32) -> Point {
        Point{x, y}
    }

    pub fn x(&self) -> u32 { self.x }
    pub fn y(&self) -> u32 { self.y }


    pub fn transform(&mut self, v:Vector) {
        self.x += v.x();
        self.y += v.y();
    }
}

#[cfg(test)]
mod tests {

    use super::{Point, Vector};

    #[test]
    fn test_fmt_display() {
        let p = Point::new(0, 0);
        assert_eq!(p.to_string(), "(0,0)");
    }

    #[test]
    fn test_ops() {
        let p1 = Point::new(1, 1);
        let p2 = Point::new(2, 2);
        let p3 = Point::new(3,3);
        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn test_transform() {
        let mut p = Point::new(0, 0);
        let tp = Point::new(1, 1);
        let v = Vector::new(1, 1);
        p.transform(v);
        assert_eq!(p, tp);
    }
}