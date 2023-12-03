use std::collections::HashSet;

fn main() {
    let res = include_str!("./four.input")
        .lines()
        .map(|line| {
            let mut set = HashSet::new();
            line.trim().split(" ").fold(true, |acc, phrase| {
                let new = set.insert(phrase);
                acc && new
            })
        })
        .fold(0, |acc, v| {
            if v {
                return acc + 1;
            }
            acc
        });

    println!("Part One: {res}");

    let res = include_str!("./four.input")
        .lines()
        .map(|line| {
            let mut set = HashSet::new();
            line.trim().split(" ").fold(true, |acc, phrase| {
                let mut chars = phrase.chars().collect::<Vec<_>>();
                chars.sort_by(|a, b| a.cmp(b));
                let sorted = String::from_iter(chars.iter());
                let new = set.insert(sorted);
                acc && new
            })
        })
        .fold(0, |acc, v| {
            if v {
                return acc + 1;
            }
            acc
        });

    println!("Part One: {res}");
}
