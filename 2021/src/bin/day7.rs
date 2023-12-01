use anyhow::Result;
use aoc::split_on_comma;

fn recursion(v: u32) -> u32 {
    if v == 0 {
        return 0;
    }
    return recursion(v - 1) + v;
}

fn find_recursive_total(v: Vec<i32>, floor: i32, ceil: i32) -> u32 {
    let mut res_floor: u32 = 0;
    let mut res_ceil: u32 = 0;
    for j in v {
        let dif = j.abs_diff(floor);
        // O(n) alternative to the recursion method I orginally found.
        res_floor += (dif * (dif + 1)) / 2;
        let dif = j.abs_diff(ceil);
        res_ceil += recursion(dif);
    }
    if res_floor < res_ceil {
        return res_floor;
    }
    res_ceil
}

fn find_total(v: Vec<i32>, median: i32) -> u32 {
    v.iter().fold(0, |acc, loc| loc.abs_diff(median) + acc)
}

fn cheapest_fuel(constant_burn: bool) -> Result<u32> {
    let mut input = split_on_comma::<i32>("./data/7.input")?;
    input.sort();

    if constant_burn {
        let median = input[input.len() / 2];
        return Ok(find_total(input, median));
    }

    let average: f32 = input.iter().sum::<i32>() as f32 / input.len() as f32;
    let floor = average.floor() as i32;
    let ceil = average.ceil() as i32;

    Ok(find_recursive_total(input, floor, ceil))
}

fn main() -> Result<()> {
    println!("Part One: {}", cheapest_fuel(true)?);
    println!("Part Two: {}", cheapest_fuel(false)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let res = cheapest_fuel(true).unwrap();
        assert_eq!(res, 342730);
    }

    #[test]
    fn part_two() {
        let res = cheapest_fuel(false).unwrap();
        assert_eq!(res, 92335207);
    }
}
