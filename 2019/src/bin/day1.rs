use anyhow::Result;
use aoc::parse_and_transform;
use num::{Float, NumCast};

fn find_fuel<F: Float>(v: F) -> F {
    (v / NumCast::from(3).unwrap()).floor() - NumCast::from(2).unwrap()
}

fn find_while<F: Float + std::ops::AddAssign>(v: F) -> F {
    let mut v = find_fuel(v);
    if v >= NumCast::from(3).unwrap() {
        let mut total = v;
        while find_fuel(v) > NumCast::from(0).unwrap() {
            total += find_fuel(v);
            v = find_fuel(v);
        }
        return total;
    } else {
        return v;
    }
}

fn main() -> Result<()> {
    let res = parse_and_transform::<f64>("data/day1.txt", find_fuel)?;

    println!("part one: {res}");

    let res2 = parse_and_transform::<f64>("data/day1.txt", find_while)?;

    println!("part two: {res2}");

    Ok(())
}
