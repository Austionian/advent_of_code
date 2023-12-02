use anyhow::anyhow;
use aoc::parse_lines;
use std::cell::RefCell;
use std::str::FromStr;

#[derive(Debug)]
struct Game {
    id: u32,
    high_blue: u32,
    high_red: u32,
    high_green: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix("Game ").ok_or(anyhow!("Invalid line!"))?;

        let (id, rest) = s.split_once(':').ok_or(anyhow!("No game id given!"))?;

        let blue = RefCell::new(Vec::new());
        let red = RefCell::new(Vec::new());
        let green = RefCell::new(Vec::new());
        rest.split(';').for_each(|m| {
            m.split(',').for_each(|r| {
                let (total, color) = r
                    .trim()
                    .split_once(" ")
                    .ok_or(anyhow!("Invalid result!"))
                    .unwrap();
                match color {
                    "blue" => {
                        let mut blue = blue.borrow_mut();
                        blue.push(total.parse().unwrap_or(0));
                    }
                    "red" => {
                        let mut red = red.borrow_mut();
                        red.push(total.parse().unwrap_or(0));
                    }
                    "green" => {
                        let mut green = green.borrow_mut();
                        green.push(total.parse().unwrap_or(0));
                    }
                    _ => unreachable!("Invalid color pulled!"),
                };
            })
        });

        Ok(Game {
            id: id.parse().unwrap(),
            high_blue: *blue.into_inner().iter().max().unwrap_or(&0),
            high_red: *red.into_inner().iter().max().unwrap_or(&0),
            high_green: *green.into_inner().iter().max().unwrap_or(&0),
        })
    }
}

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn part_one() -> anyhow::Result<u32> {
    Ok(parse_lines::<Game>("data/two.input".into())?
        .iter()
        .filter_map(|g| {
            if g.high_red > RED || g.high_green > GREEN || g.high_blue > BLUE {
                return None;
            }
            Some(g.id)
        })
        .sum())
}

fn part_two() -> anyhow::Result<u32> {
    Ok(parse_lines::<Game>("data/two.input".into())?
        .iter()
        .map(|g| g.high_red * g.high_blue * g.high_green)
        .sum())
}

fn main() -> anyhow::Result<()> {
    let res = part_one()?;
    println!("Part One: {res}");

    let res = part_two()?;
    println!("Part Two: {res}");
    Ok(())
}
