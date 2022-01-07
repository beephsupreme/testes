use std::process;

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

pub fn signatures() {
    let n: usize = 5;
    let mut values: Vec<i32> = vec![];
    let s: Segment = Segment::new(42, 61);
    let t: Segment = Segment::new(12, 62);
    let vector = mnr::get_int_vec(n);
    match vector {
        Ok(v) => {
            values = v;
        }
        Err(_) => {
            println!("FAIL: Input length mismatch.");
            process::exit(1)
        }
    }
    for v in &values {
        println!("{}", v);
    }
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
