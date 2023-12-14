use ::anyhow::Result;
use std::cmp;

fn check_vertical(s: &str, v: usize) -> bool {
    if v > s.len() {
        return false;
    }
    let front = s[..v].chars().rev().collect::<Vec<_>>();
    let back = s[v..].chars().collect::<Vec<_>>();

    let mut same = true;
    let i = cmp::min(front.len(), back.len());
    for i in 0..i {
        same = same && front[i] == back[i];
    }

    same
}

fn make_horizontal(s: &str, l: usize) -> Vec<String> {
    let mut nn = vec![Vec::new(); l];
    s.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            nn[l - i - 1].push(c);
        })
    });

    nn.iter_mut()
        .map(|v| v.iter().collect::<String>())
        .collect()
}

fn find_horizontal(input: &str, smudge: usize) -> Option<usize> {
    let length = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let collection = make_horizontal(input, length);
    for v in 1..height {
        let mut valids = Vec::new();
        for line in collection.iter() {
            if check_vertical(line.as_str(), v) {
                valids.push(v);
            }
        }
        if valids.len() == length - smudge {
            return Some(valids[0]);
        }
    }
    None
}

fn find_vertical(input: &str, smudge: usize) -> Option<usize> {
    let length = input.lines().next().unwrap().len();
    let height = input.lines().count();
    for v in 1..length {
        let mut valids = Vec::new();
        for line in input.lines() {
            if check_vertical(line, v) {
                valids.push(v);
            }
        }
        if valids.len() == height - smudge {
            return Some(valids[0]);
        }
    }
    None
}

fn part_one() -> Result<usize> {
    Ok(include_str!("../../data/thirteen.input")
        .split("\n\n")
        .fold(0, |acc, input| {
            if let Some(v) = find_vertical(input, 0) {
                return acc + v;
            } else {
                if let Some(v) = find_horizontal(input, 0) {
                    return acc + (v * 100);
                }
            }
            unreachable!("No mirror found for: {}", input);
        }))
}

fn part_two() -> Result<usize> {
    Ok(include_str!("../../data/thirteen.input")
        .split("\n\n")
        .fold(0, |acc, input| {
            if let Some(v) = find_vertical(input, 1) {
                return acc + v;
            } else {
                if let Some(v) = find_horizontal(input, 1) {
                    return acc + (v * 100);
                }
            }
            unreachable!("No mirror found for: {}", input);
        }))
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one()?);
    println!("Part Two: {}", part_two()?);

    Ok(())
}
