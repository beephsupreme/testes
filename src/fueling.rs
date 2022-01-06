/// compute_minimum_stops
/// range = how far car goes on a tank of gas
/// checkpoints starts with 0, total distance to refueling points along the route, ends with distance to destination
pub fn compute_minimum_stops(mut range: i32, checkpoints: &mut Vec<i32>) -> i32 {
    let mut refills: i32 = 0;
    let mut pos: i32 = 0;
    let dest: i32 = -1 + checkpoints.len() as i32;
    let base_range = range;
    loop {
        if pos == dest {
            return refills;
        } else if checkpoints[1 + pos as usize] > range {
            return -1;
        } else {
            while pos != dest && checkpoints[1 + pos as usize] <= range {
                pos += 1;
            }
            if pos == dest {
                continue;
            } else {
                refills += 1;
                range = base_range + checkpoints[pos as usize];
            }
        }
    }
}
