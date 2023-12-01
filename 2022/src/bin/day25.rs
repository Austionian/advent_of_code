use anyhow::Result;

fn main() -> Result<()> {
    let mut total = include_str!("./day25_input.txt")
        .lines()
        .map(|line| {
            let mut total = 0;
            for (i, c) in line.chars().rev().enumerate() {
                let v: i128 = match c {
                    '2' => 2,
                    '1' => 1,
                    '0' => 0,
                    '-' => -1,
                    '=' => -2,
                    _ => panic!("This shouldn't happen."),
                };
                total += v * (5_i128.pow(i as u32));
            }
            total
        })
        .sum::<i128>();

    let mut digs = vec![];
    while total > 0 {
        let mut r = total % 5;
        let mut q = total / 5;

        if r > 2 {
            r -= 5;
            q += 1;
        }

        digs.push(r);
        total = q
    }

    let ans = digs
        .iter()
        .rev()
        .map(|c| match c {
            0 => '0',
            1 => '1',
            2 => '2',
            -1 => '-',
            -2 => '=',
            _ => panic!("hopefully not!"),
        })
        .collect::<String>();
    println!("{:?}", ans);
    Ok(())
}
