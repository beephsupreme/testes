#[derive(Debug)]
pub struct Segment {
    x: i32,
    y: i32,
}

impl Segment {
    pub fn new(mut x: i32, mut y: i32) -> Segment {
        if x > y {
            std::mem::swap(&mut x, &mut y);
        }
        Segment { x: x, y: y }
    }

    pub fn get_y(&self) -> i32 {
        self.y
    }

    pub fn intersects(&self, obj: &Segment) -> bool {
        if self.y < obj.x {
            return false;
        } else if self.x > obj.y {
            return false;
        } else {
            return true;
        }
    }
}

pub fn cmp(lhs: &Segment, rhs: &Segment) -> bool {
    lhs.get_y() < rhs.get_y()
}
