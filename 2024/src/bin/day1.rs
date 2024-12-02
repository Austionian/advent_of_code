use std::collections::HashMap;

fn find_diff() -> usize {
    let mut left = Vec::new();
    let mut right = Vec::new();

    for line in include_str!("../../data/day1.txt").lines() {
        if let Some((left_val, right_val)) = line.split_once("   ") {
            left.push(left_val.parse::<usize>().unwrap());
            right.push(right_val.parse::<usize>().unwrap());
        }
    }

    left.sort();
    right.sort();

    left.iter()
        .zip(right)
        .fold(0, |acc, (left_val, right_val)| {
            acc + left_val.abs_diff(right_val)
        })
}

fn find_similarity() -> usize {
    let mut left = Vec::new();
    let mut map = HashMap::new();

    for line in include_str!("../../data/day1.txt").lines() {
        if let Some((left_val, right_val)) = line.split_once("   ") {
            left.push(left_val);
            map.entry(right_val)
                .and_modify(|val| *val += 1)
                .or_insert(1);
        }
    }

    left.iter().fold(0, |acc, val| {
        if let Some(map_val) = map.get(val) {
            acc + map_val * val.parse::<usize>().unwrap()
        } else {
            acc
        }
    })
}

fn main() {
    println!("Part 1: {}", find_diff());
    println!("Part 2: {}", find_similarity());
}
