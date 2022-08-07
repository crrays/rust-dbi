
use super::spoint::{Point,};



#[derive(Debug)]
pub struct Box{
    p1: Point,
    p2: Point,
}

impl Box {
    pub fn to_string(&self){
        println!("({:?}, {:?})", self.p1, self.p2);
    }
}

pub fn creat_box(x1:u32, y1:u32, x2:u32,y2:u32) -> Box {
    Box{
        p1:Point::new(x1, y1),
        p2:Point::new(x2, y2),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_box() {
        let bbox = super::creat_box(0, 0, 1, 1);
        bbox.to_string();
    }
}