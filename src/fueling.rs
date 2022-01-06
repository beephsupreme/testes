mod fueling {

    fn compute_minimum_stops(distance: i32, range: i32, checkpoints: Vec<i32>)-> i32 {
        let mut refills = 0;
        let base_range = range;
        for i in 0..checkpoints.len() {
            if i == n {
                return refills;
            } else if i + 1 > range{
                return -1;
            } else {
                while i+1 <= range && i != checkpoints.len() {
                    i += 1;
                }
                if i == checkpoints.len() {
                    continue;
                } else {
                    refills += 1;
                    range = base_range + checkpoints[i];
                }
            }
        }
        refills
    }
}