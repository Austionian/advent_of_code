use anyhow::Result;
use std::collections::HashMap;

fn two(line: &str) -> usize {
    let mut map = HashMap::new();
    for c in line.chars() {
        let cur = map.get(&c).unwrap_or(&0);
        map.insert(c, cur + 1);
    }
    for v in map.values() {
        if v == &2 {
            return 1;
        }
    }
    0
}

fn thre(line: &str) -> usize {
    let mut map = HashMap::new();
    for c in line.chars() {
        let cur = map.get(&c).unwrap_or(&0);
        map.insert(c, cur + 1);
    }
    for v in map.values() {
        if v == &3 {
            return 1;
        }
    }
    0
}

fn part_two(line: &str, line2: &str) -> Option<String> {
    let mut one_wrong = false;
    for (c1, c2) in line.chars().zip(line2.chars()) {
        if c1 != c2 {
            if one_wrong {
                return None;
            }
            one_wrong = true;
        }
    }
    Some(
        line.chars()
            .zip(line2.chars())
            .filter(|&(c1, c2)| c1 == c2)
            .map(|(c, _)| c)
            .collect(),
    )
}

fn main() -> Result<()> {
    let input: Vec<&str> = include_str!("./day2_input.txt").lines().collect();
    let mut twice = 0;
    let mut thrice = 0;
    for line in &input {
        twice += two(&line);
        thrice += thre(&line);
    }

    for i in 0..input.len() {
        for j in i + 1..input.len() {
            if let Some(common) = part_two(&input[i], &input[j]) {
                println!("Part two: {}", common);
            }
        }
    }

    println!("Part one: {}", twice * thrice);

    Ok(())
}
