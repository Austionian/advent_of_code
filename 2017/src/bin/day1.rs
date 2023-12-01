use anyhow::Result;

fn part1(input: &Vec<u32>) -> u32 {
    let mut sum = 0;
    for (i, n) in input.iter().enumerate() {
        let next = input.get(i + 1);
        match next {
            Some(v) => {
                if n == v {
                    sum += n;
                }
            }
            None => {
                if n == &input[0] {
                    sum += n;
                }
            }
        }
    }
    sum
}

fn part2(input: &Vec<u32>) -> u32 {
    let mut sum = 0;
    let half = input.len() / 2;
    for (i, n) in input.iter().enumerate() {
        if half + i > input.len() - 1 {
            if n == &input[i - half] {
                sum += n;
            }
        } else {
            if n == &input[i + half] {
                sum += n;
            }
        }
    }

    sum
}

fn main() -> Result<()> {
    let input = include_str!("./day1_input.txt")
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect::<Vec<_>>();

    println!("{:?}", part1(&input));
    println!("{}", part2(&input));
    Ok(())
}
