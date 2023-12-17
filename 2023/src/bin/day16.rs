use anyhow::{anyhow, Result};
use std::collections::HashSet;

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
struct Position {
    x: usize,
    y: usize,
    dir: Dir,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Position {
    fn get_next(&mut self, height: usize, width: usize) -> Result<()> {
        match self.dir {
            Dir::W => {
                if self.x > 0 {
                    self.x -= 1;
                } else {
                    return Err(anyhow!("Out of bounds!"));
                }
            }
            Dir::E => {
                self.x += 1;
            }
            Dir::S => {
                self.y += 1;
            }
            Dir::N => {
                if self.y > 0 {
                    self.y -= 1;
                } else {
                    return Err(anyhow!("Out of bounds!"));
                }
            }
        };

        if self.x >= width {
            return Err(anyhow!("Out of bounds!"));
        }

        if self.y >= height {
            return Err(anyhow!("Out of bounds!"));
        }

        Ok(())
    }

    fn get_dir(&mut self, layout: &Vec<Vec<char>>, splits: &mut Vec<Position>) {
        match layout[self.y][self.x] {
            '.' => {}
            '|' => match self.dir {
                Dir::N | Dir::S => {}
                Dir::E | Dir::W => {
                    splits.push(Position {
                        x: self.x,
                        y: self.y,
                        dir: Dir::S,
                    });

                    self.dir = Dir::N;
                }
            },
            '-' => match self.dir {
                Dir::E | Dir::W => {}
                Dir::N | Dir::S => {
                    splits.push(Position {
                        x: self.x,
                        y: self.y,
                        dir: Dir::W,
                    });

                    self.dir = Dir::E;
                }
            },
            '\\' => match self.dir {
                Dir::E => self.dir = Dir::S,
                Dir::W => self.dir = Dir::N,
                Dir::S => self.dir = Dir::E,
                Dir::N => self.dir = Dir::W,
            },
            '/' => match self.dir {
                Dir::E => self.dir = Dir::N,
                Dir::W => self.dir = Dir::S,
                Dir::S => self.dir = Dir::W,
                Dir::N => self.dir = Dir::E,
            },
            _ => unreachable!("Invalid layout piece found!"),
        };
    }
}

fn part_one(layout: &Vec<Vec<char>>, height: usize, width: usize) -> Result<usize> {
    let mut splits = vec![Position {
        x: 0,
        y: 0,
        dir: Dir::E,
    }];
    let mut seen = HashSet::new();
    let mut split_starts = HashSet::new();
    while splits.len() > 0 {
        let mut position = splits.pop().unwrap();
        if !split_starts.insert(position.clone()) {
            continue;
        }
        loop {
            seen.insert((position.x, position.y));

            position.get_dir(&layout, &mut splits);

            if let Err(_) = position.get_next(height, width) {
                break;
            };
        }
    }

    Ok(seen.len())
}

fn part_two(layout: &Vec<Vec<char>>, height: usize, width: usize) -> Result<usize> {
    let mut max = 0;

    for x in 0..width {
        let mut splits = vec![Position {
            x,
            y: 0,
            dir: Dir::S,
        }];
        let mut seen = HashSet::new();
        let mut split_starts = HashSet::new();
        while splits.len() > 0 {
            let mut position = splits.pop().unwrap();
            if !split_starts.insert(position.clone()) {
                continue;
            }
            loop {
                seen.insert((position.x, position.y));

                position.get_dir(layout, &mut splits);

                if let Err(_) = position.get_next(height, width) {
                    break;
                };
            }
        }

        if seen.len() > max {
            max = seen.len();
        }
    }

    for x in 0..width {
        let mut splits = vec![Position {
            x,
            y: height - 1,
            dir: Dir::N,
        }];
        let mut seen = HashSet::new();
        let mut split_starts = HashSet::new();
        while splits.len() > 0 {
            let mut position = splits.pop().unwrap();
            if !split_starts.insert(position.clone()) {
                continue;
            }
            loop {
                seen.insert((position.x, position.y));

                position.get_dir(layout, &mut splits);

                if let Err(_) = position.get_next(height, width) {
                    break;
                };
            }
        }

        if seen.len() > max {
            max = seen.len();
        }
    }

    for y in 0..height {
        let mut splits = vec![Position {
            x: 0,
            y,
            dir: Dir::E,
        }];
        let mut seen = HashSet::new();
        let mut split_starts = HashSet::new();
        while splits.len() > 0 {
            let mut position = splits.pop().unwrap();
            if !split_starts.insert(position.clone()) {
                continue;
            }
            loop {
                seen.insert((position.x, position.y));

                position.get_dir(layout, &mut splits);

                if let Err(_) = position.get_next(height, width) {
                    break;
                };
            }
        }

        if seen.len() > max {
            max = seen.len();
        }
    }

    for y in 0..height {
        let mut splits = vec![Position {
            x: width - 1,
            y,
            dir: Dir::W,
        }];
        let mut seen = HashSet::new();
        let mut split_starts = HashSet::new();
        while splits.len() > 0 {
            let mut position = splits.pop().unwrap();
            if !split_starts.insert(position.clone()) {
                continue;
            }
            loop {
                seen.insert((position.x, position.y));

                position.get_dir(layout, &mut splits);

                if let Err(_) = position.get_next(height, width) {
                    break;
                };
            }
        }

        if seen.len() > max {
            max = seen.len();
        }
    }

    Ok(max)
}

fn main() -> Result<()> {
    let layout = include_str!("../../data/sixteen.input")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = layout.len();
    let width = layout[0].len();

    println!("Part One: {}", part_one(&layout, height, width)?);
    println!("Part Two: {}", part_two(&layout, height, width)?);

    Ok(())
}
