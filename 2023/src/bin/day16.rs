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
//
// impl Add for Position {
//     type Output = Self;
//
//     fn add(self, rhs: Self) -> Self::Output {
//         Position {
//             x: self.x + rhs.x,
//             y: self.y + rhs.y,
//             dir: self.dir,
//         }
//     }
// }

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
}

fn main() -> Result<()> {
    let layout = include_str!("../../data/sixteen.input")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = layout.len();
    let width = layout[0].len();

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
            match layout[position.y][position.x] {
                '.' => {}
                '|' => match position.dir {
                    Dir::N | Dir::S => {}
                    Dir::E | Dir::W => {
                        splits.push(Position {
                            x: position.x,
                            y: position.y,
                            dir: Dir::S,
                        });

                        position.dir = Dir::N;
                    }
                },
                '-' => match position.dir {
                    Dir::E | Dir::W => {}
                    Dir::N | Dir::S => {
                        splits.push(Position {
                            x: position.x,
                            y: position.y,
                            dir: Dir::W,
                        });

                        position.dir = Dir::E;
                    }
                },
                '\\' => match position.dir {
                    Dir::E => position.dir = Dir::S,
                    Dir::W => position.dir = Dir::N,
                    Dir::S => position.dir = Dir::E,
                    Dir::N => position.dir = Dir::W,
                },
                '/' => match position.dir {
                    Dir::E => position.dir = Dir::N,
                    Dir::W => position.dir = Dir::S,
                    Dir::S => position.dir = Dir::W,
                    Dir::N => position.dir = Dir::E,
                },
                _ => unreachable!("Invalid layout piece found!"),
            };

            if let Err(_) = position.get_next(height, width) {
                break;
            };
        }
    }

    println!("{}", seen.len());

    Ok(())
}
