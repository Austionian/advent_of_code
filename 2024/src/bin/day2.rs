fn check(line: &[usize]) -> bool {
    (line.is_sorted() || line.iter().rev().is_sorted())
        && line.windows(2).fold(true, |good, window| {
            let dif = window[0].abs_diff(window[1]);
            good && (1 <= dif && dif <= 3)
        })
}

fn safety_check() -> usize {
    include_str!("../../data/day2.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .fold(0, |acc, line| if check(&line) { acc + 1 } else { acc })
}

fn tolerable_safety_check() -> usize {
    include_str!("../../data/day2.txt")
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|val| val.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .fold(0, |acc, line| {
            let mut out = false;
            for j in 0..line.len() {
                let temp = [&line[0..j], &line[j + 1..]].concat();
                if check(&temp) {
                    out = true;
                }
            }
            if out {
                acc + 1
            } else {
                acc
            }
        })
}

fn main() {
    println!("Part 1: {}", safety_check());
    println!("Part 2: {}", tolerable_safety_check());
}
