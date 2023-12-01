fn is_there_a_double(s: &str) -> bool {
    s.as_bytes()
        .windows(2)
        .fold(false, |acc, w| acc || w[0] == w[1])
}

fn is_every_number_bigger(s: &str) -> bool {
    s.as_bytes()
        .windows(2)
        .fold(true, |acc, w| acc && w[0] <= w[1])
}

fn part_one() -> u32 {
    let mut count = 0;
    for i in 231_832_u32..767_346_u32 {
        let s = format!("{i}");
        if is_there_a_double(&s) && is_every_number_bigger(&s) {
            count += 1;
        }
    }

    count
}

fn is_there_only_doubles(s: &str) -> bool {
    let mut counts = Vec::new();
    let mut prev = '\n';
    let mut count = 0;
    for (i, c) in s.chars().enumerate() {
        // first loop
        if i == 0 {
            prev = c;
            continue;
        }

        // next loop
        if prev == c {
            count += 1;
            continue;
        } else {
            prev = c;
            counts.push(count);
            count = 0;
            continue;
        }
    }

    counts.push(count);
    counts.contains(&1)
}

fn part_two() -> u32 {
    let mut count = 0;
    for i in 231_832..767_346 {
        let s = format!("{i}");
        if is_there_only_doubles(&s) && is_every_number_bigger(&s) {
            count += 1;
        }
    }

    count
}

fn main() {
    let one = part_one();
    println!("Part one: {one}");

    let two = part_two();
    println!("Part two: {two}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let res = part_one();
        assert_eq!(res, 1330);
    }
}
