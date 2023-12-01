use anyhow::Result;
use aoc::split_on_comma;
use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug)]
struct Fish {
    timer: u8,
}

impl FromStr for Fish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let timer = s.parse()?;

        Ok(Fish { timer })
    }
}

impl Fish {
    fn new() -> Fish {
        Fish { timer: 8 }
    }
}

fn lanternfish_counter(days: u32) -> Result<usize> {
    let fishs: Vec<usize> = split_on_comma("./data/6.input")?;
    let mut queue = VecDeque::from(vec![0; 9]);

    fishs.iter().for_each(|v| queue[*v] += 1);

    (0..days).for_each(|_| {
        let new_fish = queue.pop_front().unwrap();
        queue[6] += new_fish;
        queue.push_back(new_fish);
    });

    Ok(queue.iter().sum())
}

fn main() -> Result<()> {
    println!("Part One: {}", lanternfish_counter(80)?);
    println!("Part Two: {}", lanternfish_counter(256)?);

    Ok(())
}
