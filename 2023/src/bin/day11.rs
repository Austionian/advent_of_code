use anyhow::Result;

fn get_distances(galaxies: &Vec<(usize, usize)>) -> usize {
    galaxies.iter().enumerate().fold(0, |acc, (i, galaxie)| {
        let mut dif = 0;
        for j in i + 1..galaxies.len() {
            dif += usize::abs_diff(galaxie.0, galaxies[j].0)
                + usize::abs_diff(galaxie.1, galaxies[j].1)
        }
        acc + dif
    })
}

fn part_one() -> Result<usize> {
    let mut empty_y = Vec::new();
    let mut empty_x = Vec::new();
    let mut galaxies = Vec::new();
    let mut height = 0;
    include_str!("../../data/eleven.input")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            height += 1;
            let mut is_empty = true;
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    is_empty = false;
                    galaxies.push((x, y));
                }
            });
            if is_empty {
                empty_y.push(y);
            }
        });

    let width = include_str!("../../data/eleven.input")
        .lines()
        .next()
        .unwrap()
        .len();

    'outer: for x in 0..width {
        for y in 0..height {
            if galaxies.contains(&(x, y)) {
                continue 'outer;
            }
        }
        empty_x.push(x);
    }

    galaxies.iter_mut().for_each(|galaxie| {
        let mut new_y = galaxie.1;
        for y in &empty_y {
            if galaxie.1 > *y {
                new_y += 1;
            }
        }
        galaxie.1 = new_y;
        let mut new_x = galaxie.0;
        for x in &empty_x {
            if galaxie.0 > *x {
                new_x += 1;
            }
        }
        galaxie.0 = new_x;
    });

    Ok(get_distances(&galaxies))
}

fn part_two() -> Result<usize> {
    let mut empty_y = Vec::new();
    let mut empty_x = Vec::new();
    let mut galaxies = Vec::new();
    let mut height = 0;
    include_str!("../../data/eleven.input")
        .lines()
        .enumerate()
        .for_each(|(y, line)| {
            height += 1;
            let mut is_empty = true;
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    is_empty = false;
                    galaxies.push((x, y));
                }
            });
            if is_empty {
                empty_y.push(y);
            }
        });

    let width = include_str!("../../data/eleven.input")
        .lines()
        .next()
        .unwrap()
        .len();

    'outer: for x in 0..width {
        for y in 0..height {
            if galaxies.contains(&(x, y)) {
                continue 'outer;
            }
        }
        empty_x.push(x);
    }

    galaxies.iter_mut().for_each(|galaxie| {
        let mut new_y = galaxie.1;
        for y in &empty_y {
            if galaxie.1 > *y {
                new_y += 999_999;
            }
        }
        galaxie.1 = new_y;
        let mut new_x = galaxie.0;
        for x in &empty_x {
            if galaxie.0 > *x {
                new_x += 999_999;
            }
        }
        galaxie.0 = new_x;
    });

    Ok(get_distances(&galaxies))
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one()?);
    println!("Part Two: {}", part_two()?);

    Ok(())
}
