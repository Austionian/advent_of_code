use aoc::parse_lines;
use std::str::FromStr;

#[derive(Debug)]
struct Line(Vec<bool>);
struct LinePartTwo(Vec<bool>);

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line(
            s.trim()
                .chars()
                .map(|c| {
                    if c.is_digit(10) || c == '.' {
                        return false;
                    }
                    true
                })
                .collect(),
        ))
    }
}

impl FromStr for LinePartTwo {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(LinePartTwo(s.trim().chars().map(|c| c == '*').collect()))
    }
}

fn parse_line_to_vec(s: &str) -> Vec<String> {
    let mut temp_vec = Vec::new();
    let mut temp_str = String::new();
    for c in s.chars() {
        if c.is_ascii_digit() {
            temp_str.push(c);
        } else {
            if temp_str.len() >= 1 {
                temp_vec.push(temp_str.clone());
                temp_str.clear();
                temp_vec.push(c.to_string());
            } else {
                temp_vec.push(c.to_string());
            }
        }
    }
    if temp_str.len() > 0 {
        temp_vec.push(temp_str);
    }
    temp_vec
}

fn part_one() -> anyhow::Result<u32> {
    let symbols = parse_lines::<Line>("data/three.input".into())?;
    let data = include_str!("../../data/three.input")
        .lines()
        .enumerate()
        .fold(0, |acc, (y, line)| {
            let parsed_line = parse_line_to_vec(line);
            let mut x = 0;
            let mut total = 0;
            'outer: for value in parsed_line.iter() {
                if let Ok(n) = value.parse::<u32>() {
                    // left
                    if x > 1 && *symbols[y].0.get(x - 1).unwrap_or(&false) {
                        x += value.len();
                        total += n;
                        continue;
                    }
                    // right
                    if *symbols[y].0.get(x + value.len()).unwrap_or(&false) {
                        x += value.len();
                        total += n;
                        continue;
                    }
                    // upper left
                    if y > 0 && x > 0 && *symbols[y - 1].0.get(x - 1).unwrap_or(&false) {
                        x += value.len();
                        total += n;
                        continue;
                    }
                    // upper right
                    if y > 0 && *symbols[y - 1].0.get(x + value.len()).unwrap_or(&false) {
                        x += value.len();
                        total += n;
                        continue;
                    }
                    // lower left
                    if x > 0
                        && y < symbols.len() - 1
                        && *symbols[y + 1].0.get(x - 1).unwrap_or(&false)
                    {
                        x += value.len();
                        total += n;
                        continue;
                    }
                    // lower right
                    if y < symbols.len() - 1
                        && *symbols[y + 1].0.get(x + value.len()).unwrap_or(&false)
                    {
                        x += value.len();
                        total += n;
                        continue;
                    }
                    // top
                    if y > 0 {
                        for v in &symbols[y - 1].0[x..x + value.len()] {
                            if *v {
                                x += value.len();
                                total += n;
                                continue 'outer;
                            }
                        }
                    }
                    // bottom
                    if y < symbols.len() - 1 {
                        for v in &symbols[y + 1].0[x..x + value.len()] {
                            if *v {
                                x += value.len();
                                total += n;
                                continue 'outer;
                            }
                        }
                    }
                    x += value.len();
                } else {
                    x += 1;
                }
            }
            total + acc
        });
    Ok(data)
}
//
// fn part_two() -> anyhow::Result<u32> {
//     let symbols = parse_lines::<LinePartTwo>("data/three.input".into())?;
//     for (i, symbol) in symbols {}
//     Ok(0)
// }

fn main() -> anyhow::Result<()> {
    let res = part_one()?;
    println!("Part One: {res}");
    Ok(())
}
