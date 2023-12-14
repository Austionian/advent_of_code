use std::collections::HashSet;

fn main() {
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

    let res = final_positions
        .iter()
        .fold(0, |acc, rock| acc + height - rock.1);

    println!("{res}");
}
