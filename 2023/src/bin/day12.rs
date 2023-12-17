use std::collections::HashMap;

fn find_valid(
    springs: &str,
    groups: &Vec<usize>,
    index: usize,
    group_index: usize,
    length_of_current_hashes: usize,
    cache: &mut HashMap<(usize, usize, usize), usize>,
) -> usize {
    if let Some(b) = cache.get(&(index, group_index, length_of_current_hashes)) {
        return *b;
    }
    if index == springs.len() {
        if group_index == groups.len() && length_of_current_hashes == 0 {
            return 1;
        } else if group_index == groups.len() - 1 && groups[group_index] == length_of_current_hashes
        {
            return 1;
        } else {
            return 0;
        }
    }

    let mut res = 0;
    for c in ['.', '#'] {
        if springs.chars().nth(index) == Some(c) || springs.chars().nth(index) == Some('?') {
            if c == '.' && length_of_current_hashes == 0 {
                res += find_valid(springs, groups, index + 1, group_index, 0, cache);
            } else if c == '.'
                && length_of_current_hashes > 0
                && group_index < groups.len()
                && groups[group_index] == length_of_current_hashes
            {
                res += find_valid(springs, groups, index + 1, group_index + 1, 0, cache);
            } else if c == '#' {
                res += find_valid(
                    springs,
                    groups,
                    index + 1,
                    group_index,
                    length_of_current_hashes + 1,
                    cache,
                );
            }
        }
    }

    cache.insert((index, group_index, length_of_current_hashes), res);
    res
}

fn part_one() -> usize {
    include_str!("../../data/twelve.input")
        .lines()
        .fold(0, |acc, line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<_>>();
            let mut cache = HashMap::new();

            let res = find_valid(&springs, &groups, 0, 0, 0, &mut cache);
            acc + res as usize
        })
}

fn part_two() -> usize {
    include_str!("../../data/twelve.input")
        .lines()
        .fold(0, |acc, line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<_>>();

            let groups = groups
                .iter()
                .cycle()
                .take(5 * groups.len())
                .map(|v| v.clone())
                .collect::<Vec<_>>();

            let springs = vec![springs]
                .iter()
                .cycle()
                .take(5)
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join("?");

            let mut cache = HashMap::new();

            let res = find_valid(&springs, &groups, 0, 0, 0, &mut cache) as usize;
            acc + res
        })
}

fn main() {
    println!("Part One: {}", part_one());
    println!("Part Two: {}", part_two());
}
