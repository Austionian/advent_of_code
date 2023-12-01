use anyhow::Result;
use aoc2020::one_per_line;
use std::str::FromStr;

struct Password {
    min: usize,
    max: usize,
    letter: char,
    password: String,
}

impl FromStr for Password {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coll = s.split_whitespace();

        let (min, max) = coll
            .next()
            .ok_or("initial value not found.")?
            .split_once('-')
            .ok_or("no - found.")?;

        let min = min.parse().expect("couldn't parse.");
        let max = max.parse().expect("couldn't parse.");

        let letter = coll
            .next()
            .ok_or("second value not found.")?
            .chars()
            .next()
            .ok_or("no letter found.")?;

        let password = coll.next().ok_or("no password found.")?.to_string();

        Ok(Password {
            min,
            max,
            letter,
            password,
        })
    }
}

impl Password {
    fn is_valid(&self) -> bool {
        let number = self.password.matches(self.letter).count();

        self.min <= number && number <= self.max
    }

    fn is_valid_2_point_zero(&self) -> bool {
        let first = self
            .password
            .chars()
            .nth(self.min - 1)
            .expect("failed to parse")
            == self.letter;

        let second = self
            .password
            .chars()
            .nth(self.max - 1)
            .expect("failed to parse")
            == self.letter;

        (first && !second) || (second && !first)
    }
}

fn part_one() -> Result<usize> {
    Ok(one_per_line::<Password>("./data/2.input")?
        .iter()
        .filter(|pas| pas.is_valid())
        .count())
}

fn part_two() -> Result<usize> {
    Ok(one_per_line::<Password>("./data/2.input")?
        .iter()
        .filter(|pas| pas.is_valid_2_point_zero())
        .count())
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one()?);
    println!("Part Two: {}", part_two()?);
    Ok(())
}
