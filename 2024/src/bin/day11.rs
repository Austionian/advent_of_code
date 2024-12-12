use std::collections::HashMap;

const TEST: &'static str = "125 17";

fn blink(input: &str, blinks: usize) -> usize {
    let init = input
        .trim()
        .split(" ")
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();

    let mut nums = HashMap::new();

    for value in init.iter() {
        get_next(*value, 1, &mut nums);
    }

    for _ in 0..blinks {
        let mut temp_nums = HashMap::new();
        for (key, value) in nums {
            get_next(key, value, &mut temp_nums);
        }
        nums = temp_nums;
    }

    nums.iter().fold(0, |acc, (_, v)| acc + v)
}

fn get_next(key: usize, value: usize, nums: &mut HashMap<usize, usize>) {
    if key == 0 {
        nums.entry(1).and_modify(|v| *v += value).or_insert(value);
        return;
    }
    if key.to_string().len() & 1 == 0 {
        let new = key.to_string();
        let left = new[0..new.len() / 2].parse::<usize>().unwrap();
        let right = new[new.len() / 2..new.len()].parse::<usize>().unwrap();
        nums.entry(left)
            .and_modify(|v| *v += value)
            .or_insert(value);
        nums.entry(right)
            .and_modify(|v| *v += value)
            .or_insert(value);
        return;
    }

    nums.entry(key * 2024)
        .and_modify(|v| *v += value)
        .or_insert(value);
}

fn main() {
    println!(
        "Part one: {}",
        blink(include_str!("../../data/day11.txt"), 24)
    );
    println!(
        "Part two: {}",
        blink(include_str!("../../data/day11.txt"), 74)
    );
}
