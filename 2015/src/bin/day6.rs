use anyhow::Result;
use aoc2015::parse_lines;
use std::str::FromStr;

enum Ins {
    Toggle,
    Off,
    On,
}

struct Instruction {
    ins: Ins,
    ranges: Vec<(usize, usize)>,
}

fn to_ranges(s: &str) -> Vec<(usize, usize)> {
    let (from, through) = s.split_once(" through ").unwrap();

    let (from_x, from_y) = from.split_once(",").unwrap();
    let (to_x, to_y) = through.split_once(",").unwrap();
    let to_x = to_x.parse::<usize>().unwrap();
    let to_y = to_y.parse::<usize>().unwrap();
    let from_x = from_x.parse::<usize>().unwrap();
    let from_y = from_y.parse::<usize>().unwrap();

    let mut range = Vec::new();
    for i in from_y..to_y + 1 {
        let f = (1000 * i) + from_x;
        let t = (1000 * i) + to_x;
        range.push((f, t))
    }
    range
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::prelude::v1::Result<Self, Self::Err> {
        if s.starts_with("toggle") {
            let range = s.strip_prefix("toggle ").unwrap();

            return Ok(Instruction {
                ins: Ins::Toggle,
                ranges: to_ranges(range),
            });
        }

        if s.starts_with("turn off") {
            let range = s.strip_prefix("turn off ").unwrap();

            return Ok(Instruction {
                ins: Ins::Off,
                ranges: to_ranges(range),
            });
        }

        if s.starts_with("turn on") {
            let range = s.strip_prefix("turn on ").unwrap();

            return Ok(Instruction {
                ins: Ins::On,
                ranges: to_ranges(range),
            });
        }

        anyhow::bail!("Invalid instruction given!")
    }
}

fn main() -> Result<()> {
    let mut lights: Vec<u8> = vec![0; 1_000_000];

    // parse_lines::<Instruction>("src/bin/day6_input.txt".into())?
    //     .iter()
    //     .for_each(|i| {
    //         i.ranges.iter().for_each(|range| {
    //             for j in range.0..range.1 + 1 {
    //                 match i.ins {
    //                     Ins::Toggle => {
    //                         if lights[j] == 0 {
    //                             lights[j] = 1;
    //                         } else {
    //                             lights[j] = 0;
    //                         }
    //                     }
    //                     Ins::On => {
    //                         lights[j] = 1;
    //                     }
    //                     Ins::Off => {
    //                         lights[j] = 0;
    //                     }
    //                 }
    //             }
    //         });
    //     });
    //
    parse_lines::<Instruction>("src/bin/day6_input.txt".into())?
        .iter()
        .for_each(|i| {
            i.ranges.iter().for_each(|range| {
                for j in range.0..range.1 + 1 {
                    match i.ins {
                        Ins::Toggle => {
                            lights[j] += 2;
                        }
                        Ins::On => {
                            lights[j] += 1;
                        }
                        Ins::Off => {
                            if lights[j] > 0 {
                                lights[j] -= 1;
                            }
                        }
                    }
                }
            });
        });

    let res = lights.iter().fold(0, |acc, x| acc + *x as usize);
    println!("Part One: {res}");

    Ok(())
}
