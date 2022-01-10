use crate::input;
use anyhow::{Context, Result};

pub fn loot(capacity: i32, values: &mut Vec<i32>, weights: &mut Vec<i32>) -> f64 {
    let mut density: Vec<f64> = vec![];

    for i in 0..values.len() {
        density.push(values[i] as f64 / weights[i] as f64);
    }

    loot_bubble_sort(&mut density, weights, values);

    let mut optimal: f64 = 0.0;
    let mut total_weight: i32 = 0;
    let mut item_weight: i32;

    for i in 0..density.len() {
        if total_weight < capacity {
            item_weight = weights[i];
            if item_weight <= capacity - total_weight {
                total_weight += item_weight;
                optimal += item_weight as f64 * density[i];
            } else {
                item_weight = capacity - total_weight;
                total_weight += item_weight;
                optimal += item_weight as f64 * density[i];
            }
        } else {
            break;
        }
    }
    optimal
}

fn loot_bubble_sort(density: &mut Vec<f64>, weights: &mut Vec<i32>, values: &mut Vec<i32>) {
    for i in 0..density.len() {
        for j in 0..density.len() - i - 1 {
            if density[j] < density[j + 1] {
                density.swap(j, j + 1);
                weights.swap(j, j + 1);
                values.swap(j, j + 1);
            }
        }
    }
}

pub fn driver() -> Result<f64, input::InputError> {
    let mut v = match input::get_int_vec(2) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let mut weights: Vec<i32> = vec![];
    let mut values: Vec<i32> = vec![];
    for _ in 0..v[0] {
        let mut temp = match input::get_int_vec(2) {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        values.push(temp[0]);
        weights.push(temp[1]);
    }
    Ok(loot(v[1], &mut values, &mut weights))
}
