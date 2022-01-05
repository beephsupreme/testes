fn main() {
    let mut v: Vec<i32> = vec![5, 2, 4, 1, 3];
    mnr::bubble_sort(&mut v);
    println!("{:?}", v);

    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    mnr::bubble_sort(&mut strings);
    println!("{:?}", strings);
}
