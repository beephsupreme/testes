
pub fn compute_minimum_stops(mut range: i32, checkpoints: &mut Vec<i32>) -> i32 {
    let mut refills = 0;
    let base_range = range;
    for mut i in 0..checkpoints.len() {
        if i == checkpoints.len() {
            return refills;
        } else if i + 1 > range as usize {
            return -1;
        } else {
            while i + 1usize <= range as usize && i != checkpoints.len() {
                i += 1usize;
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
