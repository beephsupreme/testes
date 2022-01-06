pub fn change(mut amount: i32) -> i32 {
    let mut count: i32 = amount / 10;
    amount %= 10;
    count += amount / 5;
    count += amount % 5;
    count
}
