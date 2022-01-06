mod fueling;

fn main() {
    let mut v = vec![200, 350, 550, 900];
    println!("{}", fueling::compute_minimum_stops(2, &mut v));
    
}