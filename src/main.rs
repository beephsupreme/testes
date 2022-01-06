mod fueling;

fn main() {
    let mut v = vec![200, 350, 550, 900];
    fueling::compute_minimum_stops(900, 400, &mut v);
}