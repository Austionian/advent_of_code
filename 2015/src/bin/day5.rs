use anyhow::Result;

fn naughty(line: &str) -> usize {
    // PART 1
    // if three_vowels(line) && double(line) && !excludes(line) {
    //     return 1;
    // }
    // 0

    // PART 2
    let parsed = line.chars().collect::<Vec<_>>();
    if double_seq(&parsed) && repeat(&parsed) {
        return 1;
    }
    0
}

fn double_seq(line: &Vec<char>) -> bool {
    for (i, c) in line.iter().enumerate() {
        let current = format!("{c}{}", *line.get(i + 1).unwrap_or(&'0'));
        if line.len() > i + 2 {
            for nex in line[i + 2..].windows(2) {
                let s = format!(
                    "{}{}",
                    nex.get(0).unwrap_or(&'0'),
                    nex.get(1).unwrap_or(&'1')
                );
                if s == current {
                    return true;
                }
            }
        }
    }
    false
}

fn repeat(line: &Vec<char>) -> bool {
    for (i, c) in line.iter().enumerate() {
        if c == line.get(i + 2).unwrap_or(&'0') {
            return true;
        }
    }
    false
}

fn excludes(line: &str) -> bool {
    const EXCLUSIONS: [&str; 4] = ["ab", "cd", "pq", "xy"];

    let parsed = line.chars().collect::<Vec<_>>();

    for sl in parsed.windows(2) {
        let ss = format!("{}{}", sl[0], sl[1]);
        if EXCLUSIONS.contains(&ss.as_str()) {
            return true;
        }
    }
    false
}

fn double(line: &str) -> bool {
    for (i, c) in line.chars().enumerate() {
        if c == line.chars().nth(i + 1).unwrap_or('0') {
            return true;
        }
    }
    false
}

fn three_vowels(line: &str) -> bool {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

    let mut sum = 0;
    for c in line.chars() {
        if VOWELS.contains(&c) {
            sum += 1;
        }
        if sum == 3 {
            break;
        }
    }

    if sum >= 3 {
        return true;
    }
    false
}

fn main() -> Result<()> {
    let input: usize = include_str!("./day5_input.txt")
        .lines()
        .map(|line| naughty(line))
        .sum();
    println!("{input}");
    Ok(())
}
