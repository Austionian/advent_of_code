use anyhow::{anyhow, Result};
use aoc::parse_lines;
use std::str::FromStr;

#[derive(Debug)]
struct Plan {
    dir: String,
    len: isize,
}

impl FromStr for Plan {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        let (_, rest) = s.split_once(" ").ok_or(anyhow!("No dir found"))?;
        let (_, color) = rest.split_once(" ").ok_or(anyhow!("No dir found"))?;

        let color = color
            .strip_prefix("(#")
            .ok_or(anyhow!("No opening paren"))?
            .strip_suffix(')')
            .ok_or(anyhow!("No closing paren"))?;

        let (len, dir) = color.split_at(5);

        let len = isize::from_str_radix(format!("0{len}").as_str(), 16)?;

        let dir = match dir {
            "0" => "R",
            "1" => "D",
            "2" => "L",
            "3" => "U",
            _ => unreachable!("Invalid end digit found!"),
        };

        Ok(Plan {
            dir: dir.to_string(),
            len,
        })
    }
}

fn main() -> Result<()> {
    let mut lagoon = Vec::new();
    let mut perimeter_count = 0;

    let mut current = (1, 1);
    lagoon.push(current);
    parse_lines::<Plan>("data/eightteen.input".into())?
        .iter()
        .for_each(|plan| {
            match plan.dir.as_str() {
                "R" => {
                    current.0 += plan.len;
                    lagoon.push(current);
                    perimeter_count += plan.len;
                }
                "D" => {
                    current.1 += plan.len;
                    lagoon.push(current);
                    perimeter_count += plan.len;
                }
                "L" => {
                    current.0 -= plan.len;
                    lagoon.push(current);
                    perimeter_count += plan.len;
                }
                "U" => {
                    current.1 -= plan.len;
                    lagoon.push(current);
                    perimeter_count += plan.len;
                }
                _ => unreachable!("Invalid direction found!"),
            };
        });

    let lagoon = lagoon.into_iter().collect::<Vec<_>>();

    let mut res = lagoon.windows(2).fold(0, |acc, win| {
        let x = win[0];
        let y = win[1];
        acc + ((x.0 * y.1) - (x.1 * y.0))
    });

    let first = lagoon.first().unwrap();
    let last = lagoon.last().unwrap();
    res += (last.0 * first.1) - (last.1 * first.0);

    let ans = (res / 2) as usize + (perimeter_count as usize / 2 + 1);
    println!("Part Two: {}", ans);

    Ok(())
}
