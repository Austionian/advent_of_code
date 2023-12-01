use anyhow::Result;
use aoc2020::one_per_line;
use std::collections::VecDeque;
use std::str::FromStr;

struct Seat {
    row: u32,
    column: u32,
}

impl Seat {
    fn get_seat_id(&self) -> u32 {
        self.row * 8 + self.column
    }
}

fn lower_half(current_range: &mut VecDeque<u32>) {
    let len = current_range.len() / 2;
    current_range.drain(len..);
}

fn upper_half(current_range: &mut VecDeque<u32>) {
    let len = current_range.len() / 2;
    current_range.drain(..len);
}

fn parse_row_range(letter: char, range: &mut VecDeque<u32>) {
    match letter {
        'F' => lower_half(range),
        'B' => upper_half(range),
        _ => panic!("don't here either!"),
    }
}

fn parse_column_range(letter: char, range: &mut VecDeque<u32>) {
    match letter {
        'R' => upper_half(range),
        'L' => lower_half(range),
        _ => panic!("don't go here!"),
    }
}

impl FromStr for Seat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut row_values: VecDeque<u32> = (0..128).collect();
        let mut column_values: VecDeque<u32> = (0..8).collect();

        let row = &s[0..7];
        let col = &s[7..];

        row.chars()
            .for_each(|c| parse_row_range(c, &mut row_values));

        col.chars()
            .for_each(|c| parse_column_range(c, &mut column_values));

        Ok(Seat {
            row: row_values[0],
            column: column_values[0],
        })
    }
}

fn part_one(path: &str) -> Result<u32> {
    Ok(one_per_line::<Seat>(path)?
        .iter()
        .map(|seat| seat.get_seat_id())
        .max()
        .unwrap())
}

fn part_two(path: &str) -> Result<u32> {
    let mut seat_ids = one_per_line::<Seat>(path)?
        .iter()
        .map(|seat| seat.get_seat_id())
        .collect::<Vec<_>>();

    seat_ids.sort();

    Ok(seat_ids
        .windows(2)
        .map(|win| {
            let dif = win[0].abs_diff(win[1]);
            if dif == 2 {
                return win[0] + 1;
            }
            return 0;
        })
        .sum())
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one("./data/5.input")?);
    println!("Part Two: {}", part_two("./data/5.input")?);
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part_one_test() {
        let res = part_one("./data/5.test").unwrap();
        assert_eq!(res, 820);
    }
}
