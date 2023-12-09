use anyhow::Result;

fn part_one() -> Result<isize> {
    Ok(include_str!("../../data/nine.input")
        .lines()
        .map(|line| {
            let mut col = Vec::new();
            let values = line
                .split_whitespace()
                .filter_map(|num| num.parse::<isize>().ok())
                .collect::<Vec<_>>();

            col.push(values.clone());

            loop {
                let mut temp_values = col.last().unwrap().clone();
                temp_values = temp_values
                    .to_owned()
                    .windows(2)
                    .map(|slice| slice[1] - slice[0])
                    .collect::<Vec<_>>();

                if temp_values.last() == Some(&0) || temp_values.len() == 0 {
                    break;
                }

                col.push(temp_values.clone());
            }

            let mut last = 0;
            while col.len() > 0 {
                last += col.pop().unwrap().last().unwrap();
            }

            last
        })
        .sum::<isize>())
}

fn part_two() -> Result<isize> {
    Ok(include_str!("../../data/nine.input")
        .lines()
        .map(|line| {
            let mut col = Vec::new();
            let values = line
                .split_whitespace()
                .filter_map(|num| num.parse::<isize>().ok())
                .collect::<Vec<_>>();

            col.push(values.clone());

            loop {
                let mut temp_values = col.last().unwrap().clone();
                temp_values = temp_values
                    .to_owned()
                    .windows(2)
                    .map(|slice| slice[1] - slice[0])
                    .collect::<Vec<_>>();

                if temp_values.last() == Some(&0) || temp_values.len() == 0 {
                    break;
                }

                col.push(temp_values.clone());
            }

            let mut first = col.pop().unwrap().first().unwrap().clone();
            while col.len() > 0 {
                first = col.pop().unwrap().first().unwrap() - first;
            }

            first
        })
        .sum::<isize>())
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one()?);
    println!("Part Two: {:?}", part_two()?);

    Ok(())
}
