use anyhow::Result;
use aoc2020::one_at_a_time;
use std::collections::HashSet;

fn find_with_sum(data: &HashSet<i64>, sum: i64) -> Option<(i64, i64)> {
    for d in data {
        let wanted = sum - d;
        if data.contains(&wanted) {
            return Some((*d, wanted));
        }
    }

    None
}

fn find_with_triplet(data: &HashSet<i64>, sum: i64) -> Option<(i64, i64, i64)> {
    for d in data {
        let wanted = sum - d;
        if let Some(v) = find_with_sum(data, wanted) {
            let (x, y) = v;
            return Some((*d, x, y));
        }
    }

    None
}

fn part_one(input: &HashSet<i64>) -> Result<i64> {
    let (x, y) = find_with_sum(input, 2020).expect("No pair found.");

    Ok(x * y)
}

fn part_two(input: &HashSet<i64>) -> Result<i64> {
    let (x, y, z) = find_with_triplet(input, 2020).expect("No pair found.");

    Ok(x * y * z)
}

fn main() -> Result<()> {
    let data = one_at_a_time::<i64>("./data/1.input")?;

    println!("Part One: {}", part_one(&data)?);
    println!("Part Two: {}", part_two(&data)?);
    Ok(())
}
