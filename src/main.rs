#[derive(Debug)]
struct Segment {
    x: i32,
    y: i32,
}

impl Segment {
    fn new(x: i32, y: i32) -> Segment {
        Segment { x: x, y: y }
    }
   fn hello(&self) {
        println!("hello");
    }
    fn chicken(&self) {
        println!("butt");
    }
}


fn main() {
    let x = 32;
    let y = 42;
    let s: Segment::new(1 as i32, 2);
    s.hello();
    s.chicken();
}
