use std::collections::{HashMap, HashSet};

fn part_one(input: &str) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut anntenea: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let height = input.lines().fold(0, |acc, _| acc + 1) as i32;
    let width = input.lines().next().unwrap().len() as i32;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            if ch != '.' {
                anntenea
                    .entry(ch)
                    .and_modify(|v| v.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        })
    });

    anntenea.iter().for_each(|(_, coords)| {
        coords.iter().enumerate().for_each(|(i, coord)| {
            for j in i + 1..coords.len() {
                let coord2 = coords[j];
                let slope_y = coord2.1 - coord.1;
                let slope_x = coord2.0 - coord.0;

                let new_coord = (coord2.0 + slope_x, coord2.1 + slope_y);

                if inbounds(&new_coord, height, width) {
                    antinodes.insert(new_coord);
                }

                let new_coord2 = (coord.0 - slope_x, coord.1 - slope_y);
                if inbounds(&new_coord2, height, width) {
                    antinodes.insert(new_coord2);
                }
            }
        });
    });

    antinodes.len()
}

fn part_two(input: &str) -> usize {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut anntenea: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    let height = input.lines().fold(0, |acc, _| acc + 1) as i32;
    let width = input.lines().next().unwrap().len() as i32;

    input.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, ch)| {
            if ch != '.' {
                anntenea
                    .entry(ch)
                    .and_modify(|v| v.push((x as i32, y as i32)))
                    .or_insert(vec![(x as i32, y as i32)]);
            }
        })
    });

    anntenea.iter().for_each(|(_, coords)| {
        if coords.len() > 1 {
            coords.iter().for_each(|coord| {
                antinodes.insert(*coord);
            });
        }
        coords.iter().enumerate().for_each(|(i, coord)| {
            for j in i + 1..coords.len() {
                let coord2 = coords[j];
                let slope_y = coord2.1 - coord.1;
                let slope_x = coord2.0 - coord.0;

                let mut new_coord = (coord2.0 + slope_x, coord2.1 + slope_y);

                while inbounds(&new_coord, height, width) {
                    antinodes.insert(new_coord);
                    new_coord = (new_coord.0 + slope_x, new_coord.1 + slope_y)
                }

                let mut new_coord2 = (coord.0 - slope_x, coord.1 - slope_y);
                while inbounds(&new_coord2, height, width) {
                    antinodes.insert(new_coord2);
                    new_coord2 = (new_coord2.0 - slope_x, new_coord2.1 - slope_y);
                }
            }
        });
    });

    antinodes.len()
}

fn inbounds(c: &(i32, i32), height: i32, width: i32) -> bool {
    c.0 >= 0 && c.1 >= 0 && c.0 < width && c.1 < height
}

fn main() {
    println!(
        "Part one: {}",
        part_one(include_str!("../../data/day8.txt"))
    );
    println!(
        "Part two: {}",
        part_two(include_str!("../../data/day8.txt"))
    );
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST: &'static str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn part_one_is_correct() {
        assert_eq!(part_one(TEST), 14);
    }

    #[test]
    fn part_two_is_correct() {
        assert_eq!(part_two(TEST), 34);
    }
}
