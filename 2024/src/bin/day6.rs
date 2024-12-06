use std::{collections::HashSet, ops::Add};

const TEST: &'static str = "....#.....
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

#[derive(Clone, Debug)]
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
    if hashes
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
    visited: &mut HashSet<(i32, i32)>,
    total: &mut HashSet<(i32, i32, Dir)>,
    dir: &mut Dir,
    hashes: &HashSet<(usize, usize)>,
    height: i32,
    width: i32,
) -> bool {
    let mut visited_with_dir: HashSet<(i32, i32, Dir)> = HashSet::new();
    let mut temp = position.clone();

    visited_with_dir.insert((temp.x, temp.y, dir.clone()));

    let mut dir_p2 = dir.clone().turn();
    temp = temp + dir_p2.amount();

    while let Some(p) = get_next(&temp, hashes, &mut dir_p2, height, width) {
        temp = p.clone();
        if !visited_with_dir.insert((temp.x, temp.y, dir_p2.clone())) {
            total.insert((temp.x, temp.y, dir_p2.clone()));
            break;
        }
    }

    if let Some(p) = get_next(position, hashes, dir, height, width) {
        *position = p;
        visited.insert((position.x, position.y));
        true
    } else {
        false
    }
}

fn part_one(input: &str) -> (usize, usize) {
    let mut hashes = HashSet::new();
    let mut start = Position { x: 0, y: 0 };
    let mut dir = Dir::North;
    let width = input.lines().next().unwrap().len() as i32;
    let mut height: i32 = 0;

    let mut visited = HashSet::new();
    let mut total = HashSet::new();

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
        &mut total,
        &mut dir,
        &hashes,
        height,
        width,
    ) {}

    (visited.len(), total.len())
}

fn main() {
    println!(
        "Part one: {}",
        part_one(include_str!("../../data/day6.txt")).0
    );
    println!(
        "Part two: {}",
        part_one(include_str!("../../data/day6.txt")).1
    )
}
