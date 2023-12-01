use anyhow::Result;
use std::{collections::HashSet, num::ParseIntError, str::FromStr};

#[derive(Debug)]
struct Square {
    sets: Vec<HashSet<u32>>,
    complete: bool,
}

impl FromStr for Square {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut vec_set = Vec::new();

        let raw_square: Vec<Vec<u32>> = s
            .lines()
            .map(|line| {
                let mut row = HashSet::new();
                let parsed_row = line
                    .trim()
                    .split_whitespace()
                    .map(|num| {
                        let parsed_num = num.parse().unwrap();
                        row.insert(parsed_num);
                        parsed_num
                    })
                    .collect::<Vec<_>>();
                vec_set.push(row);
                parsed_row
            })
            .collect::<Vec<_>>();

        for col in 0..5 {
            let mut column = HashSet::new();
            for row in 0..5 {
                column.insert(raw_square[row][col]);
            }
            vec_set.push(column);
        }

        Ok(Square {
            sets: vec_set,
            complete: false,
        })
    }
}

impl Square {
    fn total(&self, multiplier: &u32) -> u32 {
        HashSet::<&u32>::from_iter(self.sets.iter().flatten())
            .into_iter()
            .sum::<u32>()
            * multiplier
    }

    fn turn(&mut self, num: &u32) -> bool {
        let mut complete = false;
        for set in self.sets.iter_mut() {
            if set.remove(num) {
                complete |= set.is_empty();
            }
        }
        complete
    }
}

fn main() -> Result<()> {
    let mut input_iter = include_str!("./day4_input.txt").split("\n\n");

    let numbers = input_iter
        .next()
        .unwrap()
        .split(',')
        .map(|c| c.parse::<u32>().expect("not a c to dig, guy!"))
        .collect::<Vec<_>>();

    let mut squares: Vec<Square> = input_iter
        .map(|square| square.parse().unwrap())
        .collect::<Vec<_>>();

    let mut winners = Vec::new();
    let mut found_first = false;
    for num in numbers {
        for square in squares.iter_mut() {
            if square.turn(&num) && !square.complete {
                if !found_first {
                    println!("Part One: {}", square.total(&num));
                    found_first = true;
                    square.complete = true;
                } else {
                    winners.push(square.total(&num));
                    square.complete = true;
                }
            }
        }
    }

    println!("Part Two: {:?}", winners.last().unwrap());

    Ok(())
}
