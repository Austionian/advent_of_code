use anyhow::Result;
use std::collections::HashSet;

fn get_next_steps(map: &Vec<Vec<char>>, p: (isize, isize)) -> Vec<(isize, isize)> {
    let dirs = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut out = Vec::new();

    for dir in dirs {
        if p.0 < 1 && dir.0 == -1 || p.1 < 1 && dir.1 == -1 {
            continue;
        }
        let new_p = (p.0 + dir.0, p.1 + dir.1);
        if let Some(v) = map.get(new_p.1 as usize) {
            if let Some(c) = v.get(new_p.0 as usize) {
                if *c == '.' || *c == 'S' {
                    out.push(new_p);
                }
            }
        }
    }

    out
}

fn main() -> Result<()> {
    let mut start = (0isize, 0isize);
    let map = include_str!("../../data/twentyone.input")
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(|(x, c)| {
                    if c == 'S' {
                        start = (x as isize, y as isize);
                    }
                    c
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut current = HashSet::new();
    current.insert(start);

    for _ in 0..64 {
        let mut temp_current = HashSet::new();

        for position in current {
            // get possible steps
            // push them into temp_current
            for new_position in get_next_steps(&map, position) {
                temp_current.insert(new_position);
            }
        }

        current = temp_current;
    }

    println!("{}", current.len());

    Ok(())
}
