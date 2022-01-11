#![allow(unused)]

use crate::input;

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
        Segment { x, y }
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
    lhs.get_y() > rhs.get_y()
}

pub fn get_segment() -> Result<Segment, input::InputError> {
    let v = match input::get_int_vec(2) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    Ok(Segment::new(v[0], v[1]))
}

pub fn bubble_sort(v: &mut Vec<Segment>) {
    for i in 0..v.len() {
        for j in 0..v.len() - i - 1 {
            if cmp(&v[j], &v[j + 1]) {
                v.swap(j, j + 1);
            }
        }
    }
}

pub fn signatures() -> Result<bool, input::InputError> {
    let n = match input::get_int() {
        Ok(n) => n,
        Err(e) => return Err(e),
    };
    let mut lines: Vec<Segment> = vec![];
    for i in 0..n {
        let mut temp = match get_segment() {
            Ok(temp) => lines.push(temp),
            Err(e) => return Err(e),
        };
    }
    bubble_sort(&mut lines);
    let mut used = vec![0; lines.len()];
    let mut found = vec![];
    let mut count = 0;
    for i in 0..lines.len() {
        if used[i] == 0 {
            for j in i + 1..lines.len() {
                if lines[i].intersects(&lines[j]) {
                    used[j] = 1;
                } else {
                    used[i] = 1;
                }
            }
            count += 1;
            found.push(lines[i].get_y());
        }
    }
    println!("{}", count);
    for i in found {
        print!("{} ", i);
    }
    println!();
    Ok(true)
}
