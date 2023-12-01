use anyhow::Result;

fn main() -> Result<()> {
    let mut collection = include_str!("./day1_input.txt")
        .split("\n\n")
        .map(|x| x.lines().flat_map(str::parse::<usize>).sum::<usize>())
        .collect::<Vec<usize>>();

    collection.sort_by(|a, b| b.cmp(a));

    println!("{:?}", collection.iter().take(3).sum::<usize>());
    println!("part one: {:?}", part_one());

    Ok(())
}

fn part_one() -> Result<()> {
    let max = include_str!("./day1_input.txt")
        .split("\n\n")
        .map(|x| {
            return x.split('\n').flat_map(str::parse::<usize>).sum::<usize>();
        })
        .max();

    println!("{:?}", max);

    Ok(())
}
