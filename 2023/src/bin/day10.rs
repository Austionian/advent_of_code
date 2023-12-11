use anyhow::{anyhow, Result};
use std::collections::HashSet;

type Map = Vec<Vec<Tile>>;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Start,
    Vertical,
    Horizontal,
    NE,
    NW,
    SW,
    SE,
    Ground,
}

impl Into<Tile> for char {
    fn into(self) -> Tile {
        match self {
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            'L' => Tile::NE,
            'J' => Tile::NW,
            '7' => Tile::SW,
            'F' => Tile::SE,
            'S' => Tile::Start,
            '.' => Tile::Ground,
            _ => unreachable!("Invalid tile found."),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn walk(map: &Map, curr: Point, dir: Dir, seen: &mut HashSet<Point>) -> Result<(Point, Dir)> {
    seen.insert(curr);
    match dir {
        Dir::Down => {
            let next = Point {
                x: curr.x,
                y: curr.y + 1,
            };
            return match map
                .get(next.y)
                .unwrap_or(&Vec::new())
                .get(next.x)
                .unwrap_or(&Tile::Ground)
            {
                Tile::NE => Ok((next, Dir::Right)),
                Tile::NW => Ok((next, Dir::Left)),
                Tile::Vertical => Ok((next, Dir::Down)),
                Tile::Horizontal => Err(anyhow!("Horizontal from up.")),
                Tile::SW => Err(anyhow!("NW from up.")),
                Tile::SE => Err(anyhow!("NE from up.")),
                Tile::Start => Err(anyhow!("END")),
                Tile::Ground => Err(anyhow!(
                    "Told to go down, found ground. cur: {:?}, next: {:?}",
                    curr,
                    next
                )),
            };
        }
        Dir::Up => {
            let next = Point {
                x: curr.x,
                y: curr.y - 1,
            };
            return match map
                .get(next.y)
                .unwrap_or(&Vec::new())
                .get(next.x)
                .unwrap_or(&Tile::Ground)
            {
                Tile::SE => Ok((next, Dir::Right)),
                Tile::SW => Ok((next, Dir::Left)),
                Tile::Vertical => Ok((next, Dir::Up)),
                Tile::Horizontal => Err(anyhow!("Horizontal from up.")),
                Tile::NW => Err(anyhow!("SW from up.")),
                Tile::NE => Err(anyhow!("SE from up.")),
                Tile::Start => Err(anyhow!("END")),
                Tile::Ground => Err(anyhow!("Told to go up, found ground.")),
            };
        }
        Dir::Right => {
            let next = Point {
                x: curr.x + 1,
                y: curr.y,
            };
            return match map
                .get(next.y)
                .unwrap_or(&Vec::new())
                .get(next.x)
                .unwrap_or(&Tile::Ground)
            {
                Tile::NE => Err(anyhow!("NE from right.")),
                Tile::NW => Ok((next, Dir::Up)),
                Tile::Horizontal => Ok((next, Dir::Right)),
                Tile::Vertical => Err(anyhow!("Horizontal from right.")),
                Tile::SW => Ok((next, Dir::Down)),
                Tile::SE => Err(anyhow!("SE from right.")),
                Tile::Start => Err(anyhow!("END")),
                Tile::Ground => Err(anyhow!("Told to go right, found ground.")),
            };
        }
        Dir::Left => {
            let next = Point {
                x: curr.x - 1,
                y: curr.y,
            };
            return match map
                .get(next.y)
                .unwrap_or(&Vec::new())
                .get(next.x)
                .unwrap_or(&Tile::Ground)
            {
                Tile::NW => Err(anyhow!("NE from right.")),
                Tile::NE => Ok((next, Dir::Up)),
                Tile::Horizontal => Ok((next, Dir::Left)),
                Tile::Vertical => Err(anyhow!("Horizontal from right.")),
                Tile::SE => Ok((next, Dir::Down)),
                Tile::SW => Err(anyhow!("SE from right.")),
                Tile::Start => Err(anyhow!("END")),
                Tile::Ground => Err(anyhow!("Told to go left, found ground.")),
            };
        }
    }
}

fn get_intersections(i: usize, j: usize, seen: &HashSet<Point>, line: &Vec<char>) -> usize {
    let mut count = 0;
    for k in 0..j {
        let point = Point { x: k, y: i };
        if !seen.contains(&point) {
            continue;
        }
        if line[k] == 'J' || line[k] == 'L' || line[k] == '|' || line[k] == 'S' {
            count += 1;
        }
    }

    count
}

fn main() -> Result<()> {
    let mut start = Point { x: 0, y: 0 };
    let map: Map = include_str!("../../data/ten.input")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start.x = x;
                        start.y = y;
                    }
                    c.into()
                })
                .collect()
        })
        .collect();

    let dirs = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];
    let mut res = 0;
    let mut seen = HashSet::new();
    for dir in dirs {
        let mut count = 1;
        let mut dir = dir;
        let mut curr = start.clone();
        let mut temp_seen = HashSet::new();
        loop {
            if let Ok((temp_curr, temp_dir)) = walk(&map, curr, dir, &mut temp_seen) {
                count += 1;
                curr = temp_curr;
                dir = temp_dir
            } else {
                break;
            }
        }
        if count > 1 {
            res = count / 2;
            seen = temp_seen;
            break;
        }
    }
    println!("Part One: {res}");

    let mut res = 0;

    for (i, line) in include_str!("../../data/ten.input").lines().enumerate() {
        for (j, _) in line.chars().enumerate() {
            let point = Point { x: j, y: i };
            let line = line.chars().collect::<Vec<_>>();
            if !seen.contains(&point) {
                let intersections = get_intersections(i, j, &seen, &line);
                if intersections > 0 {
                    if intersections % 2 == 1 {
                        res += 1;
                    }
                }
            }
        }
    }

    println!("Part Two: {res}");

    Ok(())
}
