#[derive(Debug)]
pub struct Segment {
    pub x: i32,
    pub y: i32,
}

impl Segment {
    pub fn new(x: i32, y: i32) -> Segment {
        Segment { x: x, y: y }
    }
    pub fn hello(&self) {
        println!("hello");
    }
    pub fn chicken(&self) {
        println!("butt");
    }
}
