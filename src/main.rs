mod segments;
use segments::Segment;

fn main() {
    let s: Segment = Segment::new(42 , 61);
    let t: Segment = Segment::new(12 , 62);
    println!("is {} < {}? {}", 
        s.get_y(),
        t.get_y(),
        segments::cmp(&s, &t));
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
