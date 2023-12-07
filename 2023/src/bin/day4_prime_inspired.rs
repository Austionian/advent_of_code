use anyhow::{anyhow, Result};
use aoc::parse_lines;
use std::{collections::HashMap, collections::HashSet, str::FromStr};

#[derive(Debug, Clone)]
struct Game(u32);

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.trim().split_once(":").unwrap();
        let (winning, mine) = numbers.trim().split_once("|").unwrap();
        let winning = winning
            .split_whitespace()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect::<HashSet<_>>();

        Ok(Game(
            mine.split_whitespace()
                .filter_map(|n| n.parse::<u8>().ok())
                .fold(0, |acc, n| {
                    if winning.contains(&n) {
                        return acc + 1;
                    }
                    acc
                }),
        ))
    }
}

fn part_one() -> Result<u32> {
    Ok(parse_lines::<Game>("data/four.input".into())?
        .iter()
        .fold(0, |acc, g| {
            if g.0 >= 1 {
                return acc + 2u32.pow(g.0 - 1);
            }
            acc
        }))
}

fn part_two() -> Result<usize> {
    let mut to_process = include_str!("../../data/four.input")
        .lines()
        .enumerate()
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut count = to_process.len();
    let games = parse_lines::<Game>("data/four.input".into())?
        .iter()
        .enumerate()
        .map(|(i, g)| (i.to_owned(), g.to_owned()))
        .collect::<HashMap<_, _>>();

    while let Some(v) = to_process.pop() {
        let points = &games.get(&v).ok_or(anyhow!("Value hasn't been parsed!"))?.0;

        for x in 0..*points {
            to_process.push(x as usize + v + 1);
            count += 1;
        }
    }

    Ok(count)
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one()?);

    println!("Part Two: {}", part_two()?);

    Ok(())
}
