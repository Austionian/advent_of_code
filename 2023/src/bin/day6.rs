use anyhow::Result;

fn totals(times: Vec<usize>, distances: Vec<usize>) -> Result<usize> {
    Ok(times.iter().enumerate().fold(1, |acc, (d, time)| {
        acc * (0..*time).filter(|i| i * (time - i) > distances[d]).count()
    }))
}

fn part_one() -> Result<usize> {
    let (times, distances) = include_str!("../../data/six.input")
        .trim()
        .split_once('\n')
        .unwrap();

    let times = times
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();
    let distances = distances
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();

    totals(times, distances)
}

fn part_two() -> Result<usize> {
    let (times, distances) = include_str!("../../data/six.input")
        .trim()
        .split_once('\n')
        .unwrap();

    let times = vec![times
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()];
    let distances = vec![distances
        .split_once(':')
        .unwrap()
        .1
        .split_whitespace()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()];

    totals(times, distances)
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one()?);

    println!("Part Two: {}", part_two()?);

    Ok(())
}
