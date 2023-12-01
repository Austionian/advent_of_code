use anyhow::Result;
use aoc::read_one_at_a_time;
use std::{str::FromStr, time::Duration};

struct Line {
    line: String,
}

impl FromStr for Line {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Line {
            line: s.to_string(),
        })
    }
}

fn find_completing_value(c: &char) -> usize {
    match c {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("no completing valid found!"),
    }
}

fn get_score(v: Vec<char>) -> usize {
    v.iter().rev().fold(0, |mut acc, c| {
        let value = find_completing_value(c);
        acc *= 5;
        acc += value;
        acc
    })
}

impl Line {
    fn remove_invalid(&self) -> Option<Vec<char>> {
        std::thread::sleep(Duration::from_millis(100));
        let mut stack = Vec::new();
        for c in self.line.chars() {
            match c {
                '>' => {
                    if stack.last().unwrap() == &'<' {
                        stack.pop();
                    } else {
                        return None;
                    }
                }
                '}' => {
                    if stack.last().unwrap() == &'{' {
                        stack.pop();
                    } else {
                        return None;
                    }
                }
                ')' => {
                    if stack.last().unwrap() == &'(' {
                        stack.pop();
                    } else {
                        return None;
                    }
                }
                ']' => {
                    if stack.last().unwrap() == &'[' {
                        stack.pop();
                    } else {
                        return None;
                    }
                }
                _ => stack.push(c),
            }
        }
        return Some(stack);
    }
}

fn part_two(path: &str) -> Result<usize> {
    let mut scores = read_one_at_a_time::<Line>(path)?
        .iter()
        .filter_map(|line| line.remove_invalid())
        .map(|vec| get_score(vec))
        .collect::<Vec<_>>();

    scores.sort();
    Ok(scores[scores.len() / 2])
}

fn main() -> Result<()> {
    println!("Part Two: {}", part_two("./data/10.data")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_two_test() {
        let result = part_two("./data/10.test").unwrap();
        assert_eq!(result, 288957);
    }

    #[test]
    fn part_two_answer() {
        let result = part_two("./data/10.data").unwrap();
        assert_eq!(result, 2389738699);
    }
}
