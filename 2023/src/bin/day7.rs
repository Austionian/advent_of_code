use anyhow::{anyhow, Result};
use aoc::parse_lines;
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Debug)]
struct Game {
    cards: Cards,
    bid: usize,
}

#[derive(Debug)]
struct Cards {
    strength: Strength,
    cards_raw: String,
}

#[derive(Debug)]
enum Strength {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl Strength {
    fn get_worth(&self) -> usize {
        match self {
            Strength::FiveOfAKind => 6,
            Strength::FourOfAKind => 5,
            Strength::FullHouse => 4,
            Strength::ThreeOfAKind => 3,
            Strength::TwoPair => 2,
            Strength::OnePair => 1,
            Strength::HighCard => 0,
        }
    }
}

fn get_strength(hash: HashMap<char, u8>, cards: &str) -> Strength {
    let rule = unsafe { &RULES.clone() };
    let mut modifier = &0;
    if cards.contains('J') && rule == &Some(Rule::PartTwo) {
        modifier = hash.get(&'J').unwrap();
    }
    let mut sorted = hash.iter().collect::<Vec<_>>();
    sorted.sort_by(|a, b| b.1.cmp(a.1));

    // could be a five of a kind with the same chars
    if sorted.len() == 1 {
        return Strength::FiveOfAKind;
    }

    let mut highest = sorted[0];
    let mut second = sorted[1];

    if highest.0 == &'J' && rule == &Some(Rule::PartTwo) {
        if sorted.len() > 2 {
            highest = sorted[1];
            second = sorted[2];
        } else {
            // if only two, five of a kind incoming.
            return Strength::FiveOfAKind;
        }
    }

    if highest.1 + modifier == 5 {
        return Strength::FiveOfAKind;
    }
    if highest.1 + modifier == 4 {
        return Strength::FourOfAKind;
    }
    if (highest.1 + modifier == 3 && second.1 == &2)
        || (highest.1 == &3 && second.1 + modifier == 2)
    {
        return Strength::FullHouse;
    }
    if highest.1 + modifier == 3 {
        return Strength::ThreeOfAKind;
    }
    if highest.1 == &2 && second.1 + modifier == 2 {
        return Strength::TwoPair;
    }
    if highest.1 + modifier == 2 {
        return Strength::OnePair;
    }

    Strength::HighCard
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").ok_or(anyhow!("No bid found"))?;

        let mut hash: HashMap<char, u8> = HashMap::new();
        cards.chars().for_each(|c| {
            hash.entry(c).and_modify(|count| *count += 1).or_insert(1);
        });

        let strength = get_strength(hash, &cards);

        let cards = Cards {
            strength,
            cards_raw: cards.to_string(),
        };

        Ok(Game {
            cards,
            bid: bid.parse()?,
        })
    }
}

fn get_first_card_worth(c: char) -> u32 {
    match c {
        'A' => 13,
        'K' => 12,
        'Q' => 11,
        'J' => unsafe {
            match &RULES {
                Some(Rule::PartOne) => 10,
                Some(Rule::PartTwo) => 0,
                _ => 0,
            }
        },
        'T' => 9,
        _ => c.to_digit(10).unwrap() - 1,
    }
}

fn sort_games(a: &Game, b: &Game) -> Ordering {
    if a.cards.strength.get_worth() > b.cards.strength.get_worth() {
        return Ordering::Greater;
    }
    if a.cards.strength.get_worth() < b.cards.strength.get_worth() {
        return Ordering::Less;
    }

    let mut a_first = a.cards.cards_raw.chars();
    let mut b_first = b.cards.cards_raw.chars();
    while let Some(c) = a_first.next() {
        let b_first = b_first.next().unwrap();
        if get_first_card_worth(c) > get_first_card_worth(b_first) {
            return Ordering::Greater;
        }
        if get_first_card_worth(b_first) > get_first_card_worth(c) {
            return Ordering::Less;
        }
    }
    Ordering::Equal
}

#[derive(PartialEq, Debug, Clone)]
enum Rule {
    PartOne,
    PartTwo,
}

static mut RULES: Option<Rule> = None;

fn sort_and_fold(mut games: Vec<Game>) -> usize {
    games.sort_by(sort_games);

    games
        .iter()
        .enumerate()
        .fold(0, |acc, (i, g)| (g.bid * (i + 1)) + acc)
}

fn main() -> Result<()> {
    unsafe {
        RULES = Some(Rule::PartOne);
    };
    let games = parse_lines::<Game>("data/seven.test".into())?;

    println!("Part One: {}", sort_and_fold(games));

    unsafe {
        RULES = Some(Rule::PartTwo);
    };
    let games = parse_lines::<Game>("data/seven.test".into())?;

    println!("Part Two: {}", sort_and_fold(games));

    Ok(())
}
