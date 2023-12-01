use std::collections::HashSet;

fn parse_new_location(c: &char, current_location: &mut (i32, i32), set: &mut HashSet<(i32, i32)>) {
    match c {
        '^' => current_location.0 += 1,
        'v' => current_location.0 -= 1,
        '<' => current_location.1 -= 1,
        '>' => current_location.1 += 1,
        _ => panic!("parsing didn't go well..."),
    };
    set.insert(*current_location);
}

fn find_houses() -> usize {
    let mut visited = HashSet::new();
    let mut santa_location = (0, 0);
    let mut robo_location = (0, 0);
    visited.insert(santa_location);
    for (i, c) in include_str!("./input.txt").trim().chars().enumerate() {
        if i % 2 == 0 {
            parse_new_location(&c, &mut santa_location, &mut visited);
        } else {
            parse_new_location(&c, &mut robo_location, &mut visited);
        }
    }

    visited.len()
}

fn main() {
    println!("{}", find_houses());
}
