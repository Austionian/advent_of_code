use std::str::FromStr;

struct Game {
    winning: Vec<u8>,
    mine: Vec<u8>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, numbers) = s.trim().split_once(":").unwrap();
        let (winning, mine) = numbers.trim().split_once("|").unwrap();
        let winning = winning
            .split_whitespace()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect::<Vec<_>>();
        let mine = mine
            .split_whitespace()
            .filter_map(|n| n.parse::<u8>().ok())
            .collect::<Vec<_>>();

        Ok(Game { winning, mine })
    }
}

fn main() {
    let res = include_str!("../../data/four.input")
        .lines()
        .fold(0, |acc, line| {
            let game = line.parse::<Game>().unwrap();
            let number_of_matches = game.mine.iter().fold(0, |acc, n| {
                if game.winning.contains(n) {
                    return acc + 1;
                }
                acc
            });
            if number_of_matches >= 1 {
                return acc + 2u32.pow(number_of_matches - 1);
            }
            acc
        });

    println!("Part One: {res}");

    let total_lines = include_str!("../../data/four.input")
        .lines()
        .map(|_| 1)
        .sum::<usize>();

    let mut card_totals = vec![1; total_lines];
    include_str!("../../data/four.input")
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            let game = line.parse::<Game>().unwrap();

            let number_of_matches = game.mine.iter().fold(0, |acc, n| {
                if game.winning.contains(n) {
                    return acc + 1;
                }
                acc
            });

            for _ in 0..card_totals[i] {
                for j in i + 1..i + number_of_matches + 1 {
                    card_totals[j] += 1;
                }
            }
        });

    let res = card_totals.iter().sum::<usize>();

    println!("Part Two: {res}");
}
