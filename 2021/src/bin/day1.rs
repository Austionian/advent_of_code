use anyhow::Result;
use aoc::read_one_at_a_time;

fn descending_windows(size: usize) -> Result<usize> {
    Ok(read_one_at_a_time::<u32>("./data/1.input")?
        .windows(size)
        .filter(|vs| vs[0] < vs[size - 1])
        .collect::<Vec<_>>()
        .len())
}

fn main() -> Result<()> {
    println!("Part 1: {}", descending_windows(2)?);
    println!("Part 2: {}", descending_windows(4)?);
    Ok(())
}

// let input = include_str!("./day1_input.txt")
//     .lines()
//     .map(|line| line.parse::<i32>().unwrap())
//     .collect::<Vec<_>>();
//
// let mut res = 0;
//
// for v in input.windows(2) {
//     if v[0] < v[1] {
//         res += 1;
//     }
// }
//
// let mut prev = 0;
// let mut next: i32;
// let mut part_two_res = 0;
// for i in input.windows(3) {
//     next = i.iter().sum();
//     if next > prev {
//         part_two_res += 1;
//     }
//     prev = next.clone();
// }
//
// println!("Part 1: {res}");
// // Subtract one b/c the first conparsion will always pass falsely.
// println!("Part 2: {}", part_two_res - 1);
