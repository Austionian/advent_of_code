use anyhow::Result;
use aoc::parse_lines;
use std::str::FromStr;

type ParsedHistory = Vec<Vec<isize>>;
struct History(ParsedHistory);

impl FromStr for History {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let mut col = Vec::new();
        let values = s
            .split_whitespace()
            .filter_map(|num| num.parse::<isize>().ok())
            .collect::<Vec<_>>();

        col.push(values);

        loop {
            let mut temp_values = col.last().unwrap().clone();
            temp_values = temp_values
                .to_owned()
                .windows(2)
                .map(|slice| slice[1] - slice[0])
                .collect::<Vec<_>>();

            if temp_values.last() == Some(&0) || temp_values.len() == 0 {
                break;
            }

            col.push(temp_values);
        }

        Ok(History(col))
    }
}

fn part_one() -> Result<isize> {
    Ok(parse_lines::<History>("data/nine.input".into())?
        .into_iter()
        .map(|mut history| {
            let mut last = 0;
            while history.0.len() > 0 {
                last += history.0.pop().unwrap().last().unwrap();
            }

            last
        })
        .sum())
}

fn part_two() -> Result<isize> {
    Ok(parse_lines::<History>("data/nine.input".into())?
        .into_iter()
        .map(|mut history| {
            let mut first = *history.0.pop().unwrap().first().unwrap();
            while history.0.len() > 0 {
                first = history.0.pop().unwrap().first().unwrap() - first;
            }

            first
        })
        .sum())
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one()?);
    println!("Part Two: {}", part_two()?);

    Ok(())
}
