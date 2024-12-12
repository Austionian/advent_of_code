use std::{collections::HashSet, usize};

const TEST: &'static str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

#[derive(Clone)]
struct Trailhead {
    x: i32,
    y: i32,
    value: u32,
}

fn part_one(input: &str) -> usize {
    let mut trailheads = Vec::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().fold(0, |acc, _| acc + 1);

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, ch)| {
                    if ch == '0' {
                        trailheads.push(Trailhead {
                            x: x as i32,
                            y: y as i32,
                            value: 0,
                        });
                    }
                    ch.to_digit(10)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut found = 0;
    let checks = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for trailhead in trailheads.iter() {
        let mut search = vec![trailhead.clone()];
        let mut founds = HashSet::new();
        while let Some(t) = search.pop() {
            if t.value == 9 {
                founds.insert((t.x, t.y));
                continue;
            }
            for check in checks {
                let y = t.y + check.1;
                if y < 0 || y >= height {
                    continue;
                }
                let x = t.x + check.0;
                if x < 0 || x >= width as i32 {
                    continue;
                }

                let value = map[y as usize][x as usize];
                if value == t.value + 1 {
                    search.push(Trailhead {
                        x: x as i32,
                        y: y as i32,
                        value,
                    })
                }
            }
        }
        found += founds.len();
    }

    found
}

fn part_two(input: &str) -> usize {
    let mut trailheads = Vec::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().fold(0, |acc, _| acc + 1);

    let map = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, ch)| {
                    if ch == '0' {
                        trailheads.push(Trailhead {
                            x: x as i32,
                            y: y as i32,
                            value: 0,
                        });
                    }
                    ch.to_digit(10)
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut found = 0;
    let checks = [(0, 1), (0, -1), (1, 0), (-1, 0)];

    for trailhead in trailheads.iter() {
        let mut search = vec![trailhead.clone()];
        let mut founds = Vec::new();
        while let Some(t) = search.pop() {
            if t.value == 9 {
                founds.push((t.x, t.y));
                continue;
            }
            for check in checks {
                let y = t.y + check.1;
                if y < 0 || y >= height {
                    continue;
                }
                let x = t.x + check.0;
                if x < 0 || x >= width as i32 {
                    continue;
                }

                let value = map[y as usize][x as usize];
                if value == t.value + 1 {
                    search.push(Trailhead {
                        x: x as i32,
                        y: y as i32,
                        value,
                    })
                }
            }
        }
        found += founds.len();
    }

    found
}

fn main() {
    //println!(
    //    "Part one: {}",
    //    part_one(include_str!("../../data/day10.txt"))
    //);
    println!(
        "Part two: {}",
        part_two(include_str!("../../data/day10.txt"))
    );
}
