use indicatif::ProgressBar;
use std::{collections::HashSet, ops::Add, usize};

const TEST: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    North,
    South,
    East,
    West,
}

impl Dir {
    fn amount(&self) -> Position {
        match self {
            Self::West => Position { x: -1, y: 0 },
            Self::East => Position { x: 1, y: 0 },
            Self::North => Position { x: 0, y: -1 },
            Self::South => Position { x: 0, y: 1 },
        }
    }

    fn turn(&self) -> Self {
        match self {
            Self::West => Self::North,
            Self::East => Self::South,
            Self::North => Self::East,
            Self::South => Self::West,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Position {
    x: i32,
    y: i32,
}

impl Add for Position {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: (self.x + rhs.x),
            y: (self.y + rhs.y),
        }
    }
}

fn get_next(
    position: &Position,
    hashes: &HashSet<(usize, usize)>,
    dir: &mut Dir,
    height: i32,
    width: i32,
) -> Option<Position> {
    let mut temp_position = position.clone() + dir.amount();
    while hashes
        .get(&(temp_position.x as usize, temp_position.y as usize))
        .is_some()
    {
        *dir = dir.turn();
        temp_position = dir.amount() + position.clone();
    }
    if temp_position.x < 0 || temp_position.x >= width {
        return None;
    }
    if temp_position.y < 0 || temp_position.y >= height {
        return None;
    }

    Some(temp_position)
}

fn in_bounds(
    position: &mut Position,
    visited_without: &mut HashSet<(i32, i32)>,
    visited: &mut HashSet<(i32, i32, Dir)>,
    dir: &mut Dir,
    hashes: &HashSet<(usize, usize)>,
    height: i32,
    width: i32,
) -> (bool, usize) {
    // My attempt to add an obstacle in the next point of the path and then game out the infinite
    // loops from there.
    //
    //let mut visited_with_dir: HashSet<(i32, i32, Dir)> = HashSet::new();
    //let mut temp = position.clone();
    //let mut ob = temp.clone() + dir.amount();
    //let mut dir_p = dir.clone();
    //
    //while hashes.get(&(ob.x as usize, ob.y as usize)).is_some() {
    //    dir_p = dir_p.turn();
    //    ob = temp.clone() + dir_p.amount();
    //}
    //
    //println!("ob {ob:?}");
    ////println!("start {temp:?}");
    //visited_with_dir.insert((temp.x, temp.y, dir.clone()));
    //
    //let mut dir_p2 = dir_p.clone().turn();
    //temp = temp + dir_p2.amount();
    ////println!("first {temp:?}");
    //
    //let mut temp_hashes = hashes.clone();
    //temp_hashes.insert((ob.x as usize, ob.y as usize));
    //
    ////if visited.get(&(temp.x, temp.y)).is_none() {
    //if ob != *im_start {
    //    if hashes.get(&(ob.x as usize, ob.y as usize)).is_none() {
    //        if hashes.get(&(temp.x as usize, temp.y as usize)).is_none() {
    //            while let Some(p) = get_next(&temp, &temp_hashes, &mut dir_p2, height, width) {
    //                //println!("{p:?}");
    //                temp = p.clone();
    //                if !visited_with_dir.insert((temp.x, temp.y, dir_p2.clone())) {
    //                    println!("found");
    //                    total.insert((ob.x, ob.y));
    //                    break;
    //                }
    //            }
    //        }
    //    }
    //}
    ////}

    if let Some(p) = get_next(position, hashes, dir, height, width) {
        *position = p;
        visited_without.insert((position.x, position.y));
        if !visited.insert((position.x, position.y, dir.clone())) {
            (false, 1)
        } else {
            (true, 0)
        }
    } else {
        (false, 0)
    }
}
fn part_two(input: &str) -> usize {
    let mut start = Position { x: 0, y: 0 };
    let mut hashes = HashSet::new();
    let mut dir = Dir::North;
    let width = input.lines().next().unwrap().len() as i32;
    let mut height: i32 = 0;

    let mut total = 0;

    input.lines().enumerate().for_each(|(y, line)| {
        height += 1;
        line.chars().enumerate().for_each(|(x, ch)| {
            match ch {
                '#' => {
                    hashes.insert((x, y));
                }
                '>' => {
                    start = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    dir = Dir::East;
                }
                '<' => {
                    start = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    dir = Dir::West;
                }
                '^' => {
                    start = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    dir = Dir::North;
                }
                'v' => {
                    start = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    dir = Dir::South;
                }
                _ => {}
            };
        });
    });

    let total_to_check = height * width;

    let pb = ProgressBar::new(total_to_check as u64);

    for y in 0..height {
        for x in 0..width {
            let mut new_start = start.clone();
            let mut new_hashes = hashes.clone();
            let mut new_dir = dir.clone();
            if new_hashes.insert((x as usize, y as usize)) && (x, y) != (new_start.x, new_start.y) {
                let mut visited = HashSet::new();
                visited.insert((new_start.x, new_start.y, new_dir.clone()));
                loop {
                    let (cont, found) = in_bounds(
                        &mut new_start,
                        &mut HashSet::new(),
                        &mut visited,
                        &mut new_dir,
                        &new_hashes,
                        height,
                        width,
                    );
                    if cont {
                        continue;
                    } else {
                        pb.inc(1);
                        total += found;
                        break;
                    }
                }
            }
        }
    }

    total
}

fn part_one(input: &str) -> usize {
    let mut hashes = HashSet::new();
    let mut start = Position { x: 0, y: 0 };
    let mut dir = Dir::North;
    let width = input.lines().next().unwrap().len() as i32;
    let mut height: i32 = 0;

    let mut visited = HashSet::new();

    input.lines().enumerate().for_each(|(y, line)| {
        height += 1;
        line.chars().enumerate().for_each(|(x, ch)| {
            match ch {
                '#' => {
                    hashes.insert((x, y));
                }
                '>' => {
                    start = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    dir = Dir::East;
                }
                '<' => {
                    start = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    dir = Dir::West;
                }
                '^' => {
                    start = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    dir = Dir::North;
                }
                'v' => {
                    start = Position {
                        x: x as i32,
                        y: y as i32,
                    };
                    dir = Dir::South;
                }
                _ => {}
            };
        });
    });

    visited.insert((start.x, start.y));
    while in_bounds(
        &mut start,
        &mut visited,
        &mut HashSet::new(),
        &mut dir,
        &hashes,
        height,
        width,
    )
    .0
    {}

    visited.len()
}

fn main() {
    println!(
        "Part one: {}",
        part_one(include_str!("../../data/day6.txt"))
    );
    println!(
        "Part two: {}",
        part_two(include_str!("../../data/day6.txt"))
    )
}
