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
            if cmp(&v[j], &v[j+1]){
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
    let mut s: Vec<Segment> = vec![];
    for i in 0..n {
        let mut temp = match get_segment() {
            Ok(temp) => s.push(temp),
            Err(e) => return Err(e),
        };
    }
    let it = s.iter();
    for i in it {
        println!("{:?}", i);
    }
    
    bubble_sort(&mut s);

    let it = s.iter();
    for i in it {
        println!("{:?}", i);
    }
    Ok(true)
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
