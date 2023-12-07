use anyhow::Result;
use aoc::parse_lines;
use std::str::FromStr;

struct Game(u32);

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.trim().split_once(":").unwrap();
        let (winning, mine) = numbers.trim().split_once("|").unwrap();
        let winning = winning
            .split_whitespace()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect::<Vec<_>>();

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
    let total_lines = include_str!("../../data/four.input")
        .lines()
        .map(|_| 1)
        .sum::<usize>();

    let mut card_totals = vec![1; total_lines];
    parse_lines::<Game>("data/four.input".into())?
        .iter()
        .enumerate()
        .for_each(|(i, game)| {
            for _ in 0..card_totals[i] {
                for j in i + 1..i + game.0 as usize + 1 {
                    card_totals[j] += 1;
                }
            }
        });

    Ok(card_totals.iter().sum())
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one()?);

    println!("Part Two: {}", part_two()?);

    Ok(())
}
