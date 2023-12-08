use anyhow::{anyhow, Result};
use std::collections::HashMap;

// function to find greatest common divisor
fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    if a > b {
        let c = a % b;
        return gcd(b, c);
    }
    let c = b % a;
    return gcd(a, c);
}

// function to find least common multiple
fn lcm(a: usize, b: usize) -> usize {
    (a * b) / gcd(a, b)
}

fn part_two(dirs: &Vec<char>, map: &HashMap<&str, &str>) -> Result<usize> {
    let mut positions = map
        .keys()
        .filter_map(|k| {
            if k.chars().last() == Some('A') {
                return Some(k.to_string());
            }
            None
        })
        .collect::<Vec<_>>();

    let mut i = 0;
    let mut count = 0;
    let mut cycles = Vec::new();
    while positions.len() > 0 {
        count += 1;
        let dir = dirs[i];
        positions.iter_mut().for_each(|position| {
            let p = position.to_string();
            let value = map
                .get(p.as_str())
                .ok_or(anyhow!("Key not found!"))
                .unwrap();
            let (l, r) = value
                .strip_prefix('(')
                .ok_or(anyhow!("No prefix"))
                .unwrap()
                .strip_suffix(')')
                .ok_or(anyhow!("No suffix"))
                .unwrap()
                .split_once(", ")
                .ok_or(anyhow!("No comma"))
                .unwrap();

            if dir == 'L' {
                *position = l.to_string();
            } else {
                *position = r.to_string();
            }
        });

        let mut index_to_remove = None;
        positions.iter_mut().enumerate().for_each(|(i, position)| {
            if position.chars().last() == Some('Z') {
                cycles.push(count);
                index_to_remove = Some(i);
            }
        });

        if let Some(i) = index_to_remove {
            positions.remove(i);
        }

        if i < dirs.len() - 1 {
            i += 1;
        } else {
            i = 0;
        }
    }

    Ok(cycles.iter().fold(1, |acc, v| lcm(acc, *v)))
}

fn part_one(dirs: &Vec<char>, map: &HashMap<&str, &str>) -> Result<usize> {
    let mut k = "AAA";
    let mut i = 0;
    let mut count = 0;
    while k != "ZZZ" {
        count += 1;
        let dir = dirs[i];
        let value = map.get(k).ok_or(anyhow!("Key not found!"))?;

        let (l, r) = value
            .strip_prefix('(')
            .ok_or(anyhow!("No prefix"))?
            .strip_suffix(')')
            .ok_or(anyhow!("No suffix"))?
            .split_once(", ")
            .ok_or(anyhow!("No comma"))?;

        if dir == 'L' {
            k = l;
        } else {
            k = r;
        }

        if i < dirs.len() - 1 {
            i += 1;
        } else {
            i = 0;
        }
    }

    Ok(count)
}

fn main() -> Result<()> {
    let (instructions, mappings) = include_str!("../../data/eight.input")
        .split_once("\n\n")
        .ok_or(anyhow!("No mappings found!"))?;

    let dirs = instructions.trim().chars().collect::<Vec<_>>();

    let mut map = HashMap::new();
    mappings.lines().for_each(|line| {
        let (k, v) = line.split_once(" = ").unwrap();
        map.insert(k, v);
    });

    println!("Part One: {}", part_one(&dirs, &map)?);

    println!("Part Two: {}", part_two(&dirs, &map)?);

    Ok(())
}
