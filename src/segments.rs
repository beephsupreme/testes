#![allow(unused)]

use std::process;
use std::io;
pub struct LengthMismatchError;

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

pub fn get_segment() -> Segment {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("error: unable to read user input");
    let mut tokens = input.split_whitespace();
    let p = tokens.next().unwrap().trim().parse::<i32>().unwrap();
    let q = tokens.next().unwrap().trim().parse::<i32>().unwrap();
    let s: Segment = Segment::new(p, q);
    s
}

pub fn signatures() {
    let s = get_segment();
    let t = get_segment();
    println!("{:?}", s);
    println!("{:?}", t);
}

// int main() {
//     Segment *temp;
//     vector<Segment *> lines(0);
//     vector<INT> used(0);
//     vector<INT> found(0);
//     INT count{0}, left, right, n;
//     cin >> n;
//     for (int i = 0; i < n; ++i) {
//         cin >> left >> right;
//         temp = new Segment(left, right);
//         lines.push_back(temp);
//         used.push_back(0);
//     }
//     sort(lines.begin(), lines.end(), &cmp);
//     for (int i = 0; i < lines.size(); ++i) {
//         if (!used[i]) {
//             for (int j = i + 1; j < lines.size(); ++j) {
//                 if (lines[i]->intersects(*lines[j])) {
//                     used[j] = 1;
//                 } else {
//                     used[i] = 1;
//                 }
//             }
//             count++;
//             found.push_back(lines[i]->getY());
//         }
//     }
//     cout << count << endl;
//     for (auto &f : found) {
//         cout << f << " ";
//     }
//     cout << endl;
//     return 0;
// }
