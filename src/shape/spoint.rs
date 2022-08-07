use std::fmt;

use super::svector::Vector;

#[derive(Debug)]
pub struct Point {
    x: u32,
    y: u32
}


impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
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
        println!("test point to string: {}", p.to_string());
        println!("test point display: {}", p);
    }

    #[test]
    fn test_transform() {
        let mut p = Point::new(0, 0);
        println!("point before transform: {}", p);
        let v = Vector::new(1, 1);
        p.transform(v);
        println!("point after transform: {}", p);
    }
}