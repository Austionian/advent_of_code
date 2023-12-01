use anyhow::Result;
use std::collections::{HashMap, HashSet};

fn part_one(path: &str) -> Result<usize> {
    Ok(std::fs::read_to_string(path)?
        .split("\n\n")
        .map(|answers| {
            let mut set = HashSet::new();
            let _ = answers
                .lines()
                .map(|line| {
                    let _ = line.chars().map(|c| set.insert(c)).collect::<Vec<_>>();
                })
                .collect::<Vec<_>>();
            set.len()
        })
        .sum())
}

fn part_two(path: &str) -> Result<usize> {
    Ok(std::fs::read_to_string(path)?
        .split("\n\n")
        .map(|answers| {
            let mut map = HashMap::new();
            let mut answer_count = 0;
            let _ = answers
                .lines()
                .map(|line| {
                    answer_count += 1;
                    let _ = line
                        .chars()
                        .map(|c| {
                            map.entry(c).and_modify(|v| *v += 1).or_insert(1);
                        })
                        .collect::<Vec<_>>();
                })
                .collect::<Vec<_>>();
            let ans: Vec<_> = map.iter_mut().filter(|v| v.1 == &answer_count).collect();
            ans.len()
        })
        .sum())
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one("./data/6.input")?);
    println!("Part Two: {}", part_two("./data/6.input")?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let res = part_one("./data/6.test").unwrap();
        assert_eq!(res, 11);
    }

    #[test]
    fn part_two_test() {
        let res = part_two("./data/6.test").unwrap();
        assert_eq!(res, 6);
    }
}
