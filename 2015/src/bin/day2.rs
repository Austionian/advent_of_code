use anyhow::Result;

fn get_volume(dims: &Vec<usize>) -> usize {
    dims.iter().fold(1, |acc, x| acc * x)
}

fn main() -> Result<()> {
    let result: usize = include_str!("./day2_input.txt")
        .lines()
        .map(|line| {
            let mut dims = line
                .split('x')
                .map(|v| v.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            let vol = get_volume(&dims);

            dims.sort();
            (dims[0] * 2) + (dims[1] * 2) + vol
        })
        .sum();

    println!("{}", result);
    Ok(())
}
