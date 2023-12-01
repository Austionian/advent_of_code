use anyhow::Result;

fn part1(input: &mut Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    input.iter_mut().for_each(|v| {
        v.sort();
        sum += v[v.len() - 1] - v[0];
    });
    sum
}

fn part2(input: Vec<Vec<usize>>) -> usize {
    let mut sum = 0;
    input.iter().for_each(|v| {
        for (i, n) in v.iter().enumerate() {
            for (j, x) in v.iter().enumerate() {
                if i == j {
                    continue;
                }
                if n % x == 0 {
                    sum += n / x;
                    break;
                }
            }
        }
    });
    sum
}

fn main() -> Result<()> {
    let mut input = include_str!("./day2_input.txt")
        .lines()
        .map(|line| {
            line.split('\t')
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("\nPart 1: {}", part1(&mut input));
    println!("-------------------");
    println!("Part 2: {}", part2(input));
    Ok(())
}
