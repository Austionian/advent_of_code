use anyhow::Result;
use std::collections::HashMap;

struct Power {
    gamma: String,
    epsilon: String,
}

impl Power {
    fn answer(&self) {
        let g = u32::from_str_radix(&self.gamma, 2).unwrap();
        let e = u32::from_str_radix(&self.epsilon, 2).unwrap();
        println!("{}", g * e);
    }
}

fn part_two(vs: Vec<Vec<u32>>, start: usize, oxy: bool) -> String {
    if vs.len() == 1 {
        let res: String = vs[0].iter().map(|v| v.to_string()).collect();
        return res;
    }

    let new_start = start + 1;

    let mut zero = Vec::new();
    let mut one = Vec::new();

    for i in vs {
        if i[start] == 1 {
            one.push(i);
        } else {
            zero.push(i);
        }
    }

    if oxy {
        if zero.len() > one.len() {
            return part_two(zero, new_start, true);
        }
        return part_two(one, new_start, true);
    }
    if zero.len() > one.len() {
        if one.len() >= 1 {
            return part_two(one, new_start, false);
        }
    }
    return part_two(zero, new_start, false);
}

fn main() -> Result<()> {
    let mut map = HashMap::new();
    let total_lines = include_str!("./day3_input.txt")
        .lines()
        .map(|line| {
            for (i, c) in line.chars().enumerate() {
                let cur = map.get(&i).unwrap_or(&0);
                let num = c.to_digit(10).unwrap();
                let new = cur + num;
                map.insert(i, new);
            }
            1
        })
        .sum::<u32>();

    let mut power = Power {
        gamma: String::new(),
        epsilon: String::new(),
    };

    // itering through a hash map does not garuntee order!!
    for i in 0..12 {
        let v = map.get(&i).unwrap();
        if v > &500 {
            power.gamma.push('1');
            power.epsilon.push('0');
        } else {
            power.gamma.push('0');
            power.epsilon.push('1');
        }
    }

    print!("Part One: ");
    power.answer();

    let input: Vec<Vec<u32>> = include_str!("./day3_input.txt")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let mut zero = Vec::new();
    let mut one = Vec::new();

    for i in input {
        if i[0] == 1 {
            one.push(i);
        } else {
            zero.push(i);
        }
    }

    let oxy;
    let co;

    if zero.len() > one.len() {
        oxy = part_two(zero, 0, true);
        co = part_two(one, 0, false);
    } else {
        oxy = part_two(one, 0, true);
        co = part_two(zero, 0, false);
    }

    let life_support = Power {
        gamma: oxy,
        epsilon: co,
    };

    print!("Part Two: ");
    life_support.answer();

    Ok(())
}
