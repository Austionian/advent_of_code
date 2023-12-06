use aoc::parse_lines;
use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Line(Vec<bool>);

#[derive(Debug)]
struct Num {
    value: u32,
    locations: HashSet<(usize, usize)>,
}

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

fn get_surrounding_location(len: usize, x: usize, y: usize) -> HashSet<(usize, usize)> {
    let mut set = HashSet::new();

    if x > 0 && y > 0 {
        set.insert((x - 1, y - 1));
    }
    if x > 0 {
        set.insert((x - 1, y + 1));

        set.insert((x - 1, y));
    }

    set.insert((x + len, y));
    if y > 0 {
        for i in x..x + len + 1 {
            set.insert((i, y - 1));
        }
    };

    for i in x..x + len + 1 {
        set.insert((i, y + 1));
    }

    set
}

fn part_two() -> anyhow::Result<u32> {
    let nums = include_str!("../../data/three.input")
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let parsed = parse_line_to_vec(line);
            let mut x = 0;
            parsed
                .iter()
                .filter_map(|v| {
                    let y = y.clone();
                    if v.parse::<u32>().is_ok() {
                        let locations = get_surrounding_location(v.len(), x, y);
                        x += v.len();
                        return Some(Num {
                            value: v.clone().parse::<u32>().unwrap(),
                            locations,
                        });
                    }
                    x += 1;
                    None
                })
                .collect::<Vec<Num>>()
        })
        .collect::<Vec<_>>();

    Ok(include_str!("../../data/three.input")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == '*' {
                        let mut values = (0, 0);
                        for num in &nums {
                            if num.locations.contains(&(x, y)) {
                                if values.0 == 0 {
                                    values.0 = num.value;
                                } else {
                                    values.1 = num.value;
                                }
                            }
                        }
                        return values.0 * values.1;
                    }
                    0
                })
                .sum::<u32>()
        })
        .sum::<u32>())
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

fn main() -> anyhow::Result<()> {
    let res = part_one()?;
    println!("Part One: {res}");

    let res = part_two()?;
    println!("Part Two: {res}");

    Ok(())
}
