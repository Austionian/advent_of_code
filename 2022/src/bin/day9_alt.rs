use anyhow::Result;
use std::collections::HashSet;

fn lead(dir: &str, head: &mut (i32, i32)) {
    match dir {
        "R" => head.0 += 1,
        "L" => head.0 -= 1,
        "U" => head.1 += 1,
        "D" => head.1 -= 1,
        _ => unreachable!("no way jose!"),
    }
}

fn follow(leader: (i32, i32), follower: &mut (i32, i32)) {
    let (dx, dy) = (leader.0 - follower.0, leader.1 - follower.1);
    if dx.abs() > 1 || dy.abs() > 1 {
        follower.0 += dx.signum();
        follower.1 += dy.signum();
    }
}

fn visit(know_num: usize) -> usize {
    let mut rope = vec![(0, 0); know_num];
    let mut visited = HashSet::from([(0, 0)]);

    include_str!("./day9_input.txt").lines().for_each(|line| {
        let (dir, steps) = line.split_once(' ').unwrap();
        let steps = steps.parse::<u32>().unwrap();

        for _ in 0..steps {
            lead(dir, &mut rope[0]);

            for i in 1..rope.len() {
                follow(rope[i - 1], &mut rope[i]);
            }

            visited.insert(*rope.last().unwrap());
        }
    });

    visited.len()
}

fn main() -> Result<()> {
    println!("{}", visit(10));
    Ok(())
}
