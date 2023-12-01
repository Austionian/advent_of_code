use anyhow::Result;
use aoc::read_one_at_a_time;
use std::str::FromStr;

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

fn find_value(c: &char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid invalid character given!"),
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
    fn is_valid(&self) -> Option<char> {
        let mut stack = Vec::new();
        for c in self.line.chars() {
            match c {
                '>' => {
                    if stack.last().unwrap() == &'<' {
                        stack.pop();
                    } else {
                        return Some(c);
                    }
                }
                '}' => {
                    if stack.last().unwrap() == &'{' {
                        stack.pop();
                    } else {
                        return Some(c);
                    }
                }
                ')' => {
                    if stack.last().unwrap() == &'(' {
                        stack.pop();
                    } else {
                        return Some(c);
                    }
                }
                ']' => {
                    if stack.last().unwrap() == &'[' {
                        stack.pop();
                    } else {
                        return Some(c);
                    }
                }
                _ => stack.push(c),
            }
        }
        return None;
    }

    fn remove_invalid(&self) -> Option<Vec<char>> {
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

fn part_one(path: &str) -> Result<u32> {
    Ok(read_one_at_a_time::<Line>(path)?
        .iter()
        .filter_map(|line| line.is_valid())
        .map(|e| find_value(&e))
        .sum())
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
    println!("Part One: {}", part_one("./data/10.data")?);
    println!("Part Two: {}", part_two("./data/10.data")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let result = part_one("./data/10.test").unwrap();
        assert_eq!(result, 26397);
    }

    #[test]
    fn part_one_answer() {
        let result = part_one("./data/10.data").unwrap();
        assert_eq!(result, 318099);
    }

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
