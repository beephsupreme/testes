mod maxloot;

fn main() {
    let capacity: i32 = 50;
    let mut values: Vec<i32> = vec![60, 100, 120];
    let mut weights: Vec<i32> = vec![20, 50, 30];
    let result = maxloot::loot(capacity, &mut values, &mut weights);
    println!("{}", result);
}
