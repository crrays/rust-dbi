use std::fmt;


pub struct Vector {
    x: u32,
    y: u32
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Vector {
    pub fn new(x:u32, y:u32) -> Vector {
        Vector{x, y}
    }

    pub fn x(&self) -> u32 { self.x }
    pub fn y(&self) -> u32 { self.y }

    pub fn transform(&mut self) {
        self.x += 1;
        self.y += 1;
    }
}