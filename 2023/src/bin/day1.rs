use anyhow::Result;
use aoc::parse_lines;
use std::str::FromStr;

struct Calibration(u32);
struct TwoCalibration(u32);

impl FromStr for Calibration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let nums = s
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect::<Vec<u32>>();

        Ok(Calibration(
            nums.first().unwrap() * 10 + nums.last().unwrap(),
        ))
    }
}

const CASES: [&str; 18] = [
    "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
    "seven", "eight", "nine",
];

fn find_last(s: &str) -> u32 {
    let rev = s.chars().rev().collect::<String>();
    get_u32(
        &CASES
            .iter()
            .filter_map(|case| {
                let rev_case = case.chars().rev().collect::<String>();
                if rev.find(&rev_case).is_some() {
                    return Some((case, rev.find(&rev_case).unwrap()));
                }
                None
            })
            .min_by(|x, y| x.1.cmp(&y.1))
            .unwrap()
            .0,
    )
}

fn find_first(s: &str) -> u32 {
    get_u32(
        CASES
            .iter()
            .filter_map(|case| {
                if s.find(case).is_some() {
                    return Some((*case, s.find(case).unwrap()));
                }
                None
            })
            .min_by(|x, y| x.1.cmp(&y.1))
            .unwrap()
            .0,
    )
}

impl FromStr for TwoCalibration {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(TwoCalibration(find_first(s) * 10 + find_last(s)))
    }
}

fn get_u32(s: &str) -> u32 {
    match s {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => s.chars().next().unwrap().to_digit(10).unwrap(),
    }
}

fn main() -> Result<()> {
    let part_one = parse_lines::<Calibration>("data/one.input".into())?
        .iter()
        .fold(0, |acc, c| acc + c.0);

    println!("Part One: {}", part_one);

    let part_two = parse_lines::<TwoCalibration>("data/one.input".into())?
        .iter()
        .fold(0, |acc, c| acc + c.0);

    println!("Part Two: {}", part_two);

    Ok(())
}
