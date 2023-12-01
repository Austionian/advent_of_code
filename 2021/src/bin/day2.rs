use anyhow::Result;
use std::{fmt, str::FromStr};

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "x: {}, y: {}", self.x, self.y)?;
        Ok(())
    }
}

impl FromStr for Position {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, amt) = s.split_once(' ').expect("Bad line given.");
        let amt = amt.parse::<i32>()?;
        match dir {
            "forward" => Ok(Position { x: amt, y: 0 }),
            "up" => Ok(Position { x: 0, y: -amt }),
            "down" => Ok(Position { x: 0, y: amt }),
            _ => panic!("unable to parse this position"),
        }
    }
}

impl Position {
    fn final_result(&self) -> i32 {
        self.x * self.y
    }
}

fn main() -> Result<()> {
    let mut start = Position { x: 0, y: 0 };

    let mut part_two_start = Position { x: 0, y: 0 };
    let mut aim = 0;

    for line in include_str!("./day2_input.txt").lines() {
        let adv = line.parse::<Position>().unwrap();

        // PART ONE CALC
        start.x += adv.x;
        start.y += adv.y;

        // PART TWO CALC
        part_two_start.x += adv.x;
        aim += adv.y;
        part_two_start.y += aim * adv.x;
    }

    println!("Part 1: {}", start.final_result());
    println!("Part 2: {}", part_two_start.final_result());

    Ok(())
}
