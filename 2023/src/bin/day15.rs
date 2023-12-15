use anyhow::{anyhow, bail};
use std::path::PathBuf;
use std::str::FromStr;
use std::{collections::HashMap, fs};

fn hash(s: &str) -> usize {
    s.chars().fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

type Box = HashMap<usize, Vec<(String, usize)>>;

struct Boxes(Box);

enum Instruction {
    AddOrUpdate(String, usize),
    Remove(String),
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains('=') {
            let (label, value) = s.split_once('=').ok_or(anyhow!("No equals sign found!"))?;
            let value = value.parse()?;
            return Ok(Instruction::AddOrUpdate(label.to_string(), value));
        }

        Ok(Instruction::Remove(
            s.split('-')
                .next()
                .ok_or(anyhow!("No dash found!"))?
                .to_string(),
        ))
    }
}

impl Boxes {
    fn new() -> Self {
        Boxes(HashMap::with_capacity(256))
    }

    fn calculate_focusing_power(self) -> anyhow::Result<usize> {
        Ok(self
            .0
            .into_iter()
            .map(|(k, t)| {
                t.iter()
                    .enumerate()
                    .map(|(j, v)| (k + 1) * (j + 1) * v.1)
                    .sum::<usize>()
            })
            .sum())
    }

    fn parse_instructions(&mut self, path: PathBuf) -> anyhow::Result<()> {
        for instruction in fs::read_to_string(path)?.trim().split(",") {
            match instruction.parse() {
                Ok(Instruction::AddOrUpdate(label, value)) => {
                    let hash = hash(&label);
                    self.0
                        .entry(hash)
                        .and_modify(|v: &mut Vec<(String, usize)>| {
                            let mut found = false;
                            for t in v.iter_mut() {
                                if t.0 == label.to_string() {
                                    t.1 = value;
                                    found = true;
                                }
                            }
                            if !found {
                                v.push((label.to_string(), value));
                            }
                        })
                        .or_insert(vec![(label.to_string(), value)]);
                }
                Ok(Instruction::Remove(label)) => {
                    let hash = hash(&label);
                    self.0.entry(hash).and_modify(|v| {
                        for (i, t) in v.iter_mut().enumerate() {
                            if t.0 == label {
                                v.remove(i);
                                break;
                            }
                        }
                    });
                }
                Err(e) => bail!("Unable to parse instruction: {}, err: {}", instruction, e),
            };
        }

        Ok(())
    }
}

fn part_one() -> anyhow::Result<usize> {
    Ok(include_str!("../../data/fifteen.test")
        .trim()
        .split(",")
        .map(hash)
        .sum::<usize>())
}

fn part_two() -> anyhow::Result<usize> {
    let mut boxes = Boxes::new();

    boxes.parse_instructions("data/fifteen.input".into())?;

    boxes.calculate_focusing_power()
}

fn main() -> anyhow::Result<()> {
    println!("Part One: {}", part_one()?);
    println!("Part Two: {}", part_two()?);

    Ok(())
}
