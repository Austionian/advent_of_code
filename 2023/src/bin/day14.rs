use anyhow::Result;
use std::collections::{BTreeSet, HashMap, HashSet};

fn cycle(
    rocks: BTreeSet<(usize, usize)>,
    cubes: &BTreeSet<(usize, usize)>,
    cache: &mut HashMap<BTreeSet<(usize, usize)>, BTreeSet<(usize, usize)>>,
    length: usize,
    height: usize,
) -> BTreeSet<(usize, usize)> {
    if let Some(v) = cache.get(&rocks) {
        return v.clone();
    } else {
        // north
        let mut n = BTreeSet::new();
        rocks.iter().for_each(|rock| {
            let mut new_position = rock.clone();
            loop {
                if new_position.1 == 0 {
                    while !n.insert(new_position) {
                        new_position.1 += 1;
                    }
                    break;
                }
                if n.contains(&(new_position.0, new_position.1 - 1)) {
                    while !n.insert(new_position) {
                        new_position.1 += 1;
                    }
                    break;
                }
                if cubes.contains(&(new_position.0, new_position.1 - 1)) {
                    while !n.insert(new_position) {
                        new_position.1 += 1;
                    }
                    break;
                }
                new_position.1 -= 1;
            }
        });

        // west
        let mut w = BTreeSet::new();
        n.iter().for_each(|rock| {
            let mut new_position = rock.clone();
            loop {
                if new_position.0 == 0 {
                    while !w.insert(new_position) {
                        new_position.0 += 1;
                    }
                    break;
                }
                if w.contains(&(new_position.0 - 1, new_position.1)) {
                    while !w.insert(new_position) {
                        new_position.0 += 1;
                    }
                    break;
                }
                if cubes.contains(&(new_position.0 - 1, new_position.1)) {
                    while !w.insert(new_position) {
                        new_position.0 += 1;
                    }
                    break;
                }
                new_position.0 -= 1;
            }
        });

        // south
        let mut s = BTreeSet::new();
        w.iter().for_each(|rock| {
            let mut new_position = rock.clone();
            loop {
                if new_position.1 == height - 1 {
                    while !s.insert(new_position) {
                        new_position.1 -= 1;
                    }
                    break;
                }
                if s.contains(&(new_position.0, new_position.1 + 1)) {
                    while !s.insert(new_position) {
                        new_position.1 -= 1;
                    }
                    break;
                }
                if cubes.contains(&(new_position.0, new_position.1 + 1)) {
                    while !s.insert(new_position) {
                        new_position.1 -= 1;
                    }
                    break;
                }
                new_position.1 += 1;
            }
        });

        // east
        let mut e = BTreeSet::new();
        s.iter().for_each(|rock| {
            let mut new_position = rock.clone();
            loop {
                if new_position.0 == length {
                    while !e.insert(new_position) {
                        new_position.0 -= 1;
                    }
                    break;
                }
                if e.contains(&(new_position.0 + 1, new_position.1)) {
                    while !e.insert(new_position) {
                        new_position.0 -= 1;
                    }
                    break;
                }
                if cubes.contains(&(new_position.0 + 1, new_position.1)) {
                    while !e.insert(new_position) {
                        new_position.0 -= 1;
                    }
                    break;
                }
                new_position.0 += 1;
            }
        });

        cache.entry(rocks).or_insert(e.clone());

        e
    }
}

fn part_one() -> Result<usize> {
    let mut rocks = Vec::new();
    let mut cubes = HashSet::new();
    let mut height = 0;
    include_str!("../../data/fourteen.input")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, v)| {
                if v == 'O' {
                    rocks.push((x, y));
                }
                if v == '#' {
                    cubes.insert((x, y));
                }
            });
            height = y + 1;
        });

    let mut final_positions = HashSet::new();
    rocks.iter().for_each(|rock| {
        let mut new_position = rock.clone();
        loop {
            if new_position.1 == 0 {
                final_positions.insert(new_position);
                break;
            }
            if final_positions.contains(&(new_position.0, new_position.1 - 1)) {
                final_positions.insert(new_position);
                break;
            }
            if cubes.contains(&(new_position.0, new_position.1 - 1)) {
                final_positions.insert(new_position);
                break;
            }
            new_position.1 -= 1;
        }
    });

    Ok(final_positions
        .iter()
        .fold(0, |acc, rock| acc + height - rock.1))
}

fn part_two() -> Result<usize> {
    let mut rocks = BTreeSet::new();
    let mut cubes = BTreeSet::new();
    let mut height = 0;
    let mut length = 0;
    include_str!("../../data/fourteen.input")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, v)| {
                if v == 'O' {
                    rocks.insert((x, y));
                }
                if v == '#' {
                    cubes.insert((x, y));
                }
                length = x;
            });
            height = y + 1;
        });

    let mut cache = HashMap::new();
    for _ in 0..1_000 {
        rocks = cycle(rocks, &cubes, &mut cache, length, height);
    }

    Ok(rocks.iter().fold(0, |acc, rock| acc + height - rock.1))
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one()?);
    println!("Part Two: {}", part_two()?);

    Ok(())
}
