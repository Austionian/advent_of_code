use anyhow::Result;

fn main() -> Result<()> {
    let input = aoc::two_d_vec("./data/9.input")?;

    let row_len = input.len();
    let col_len = input[0].len();

    let mut ans = Vec::new();

    for (row, line) in input.iter().enumerate() {
        line.iter().enumerate().for_each(|(col, val)| {
            if aoc::cardinal_directions(row, col, row_len, col_len)
                .into_iter()
                .all(|(x, y)| input[x][y] > *val)
            {
                ans.push(input[row][col].clone());
            }
        })
    }

    let answer = ans.iter().sum::<u32>() + ans.len() as u32;

    println!("Part One: {answer}");

    Ok(())
}
