mod day4;

use anyhow::{anyhow, Result};
use aoc::parse_lines;
use day4::Game;
use std::collections::HashMap;

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
    println!("Part Two: {}", part_two()?);

    Ok(())
}
