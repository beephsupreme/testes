/// compute_minimum_stops
/// distance = 
/// range = how far car goes on a tank of gas
/// checkpoints are distances to gas stations from starting point (0)
pub fn compute_minimum_stops(distance: i32, mut range: i32, refuel_points: &mut Vec<i32>) -> i32 {
    let mut rp:Vec<i32> = vec![0];
    rp.append(refuel_points);
    rp.push(distance);
    let mut refills: i32 = 0;
    let mut pos: i32 = 0;
    let dest: i32 = -1 + rp.len() as i32;
    let base_range = range;
    loop {
        if pos == dest {
            return refills;
        } else if rp[1 + pos as usize] > range {
            return -1;
        } else {
            while pos != dest && rp[1 + pos as usize] <= range {
                pos += 1;
            }
            if pos == dest {
                continue;
            } else {
                refills += 1;
                range = base_range + rp[pos as usize];
            }
        }
    }
}
