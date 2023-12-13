fn is_valid(springs: &str, groups: &Vec<usize>) -> bool {
    let mut current = 0;
    let mut seen = Vec::new();
    for c in springs.chars() {
        if c == '.' {
            if current > 0 {
                seen.push(current);
            }
            current = 0;
        } else if c == '#' {
            current += 1;
        } else {
            return false;
        }
    }
    if current > 0 {
        seen.push(current);
    }

    seen.iter()
        .zip(groups)
        .fold(true, |acc, (a, b)| acc && a == b)
        && seen.len() == groups.len()
}

fn find_valid(springs: &str, groups: &Vec<usize>, i: usize) -> usize {
    if i == springs.len() {
        if is_valid(springs, groups) {
            return 1;
        }
        return 0;
    }
    if springs.chars().nth(i) == Some('?') {
        let new_str = springs[..i].to_string() + "#" + &springs[i + 1..].to_string();
        let other_str = springs[..i].to_string() + "." + &springs[i + 1..].to_string();
        return find_valid(&new_str, groups, i + 1) + find_valid(&other_str, groups, i + 1);
    }
    return find_valid(springs, groups, i + 1);
}

fn main() {
    let res = include_str!("../../data/twelve.input")
        .lines()
        .fold(0, |acc, line| {
            let (springs, groups) = line.split_once(' ').unwrap();
            let groups = groups
                .split(',')
                .filter_map(|v| v.parse::<usize>().ok())
                .collect::<Vec<_>>();

            let res = find_valid(&springs, &groups, 0);
            acc + res
        });

    println!("Part One: {res}");
}
