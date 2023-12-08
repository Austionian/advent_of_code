use anyhow::{anyhow, Result};
use aoc::parse_lines;
use std::{cmp::Ordering, collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Game {
    cards: Cards,
    bid: usize,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(" ").ok_or(anyhow!("No bid found"))?;

        let mut hash: HashMap<char, u8> = HashMap::new();
        cards.chars().for_each(|c| {
            hash.entry(c).and_modify(|count| *count += 1).or_insert(1);
        });

        let cards = Cards {
            strength: hash.into(),
            raw_cards: cards.chars().map(|c| c.into()).collect(),
        };

        Ok(Game {
            cards,
            bid: bid.parse()?,
        })
    }
}

impl Ord for Game {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.cards.strength > other.cards.strength {
            return Ordering::Greater;
        }
        if self.cards.strength < other.cards.strength {
            return Ordering::Less;
        }

        let mut a_first = self.cards.raw_cards.iter();
        let mut b_first = other.cards.raw_cards.iter();
        while let Some(c) = a_first.next() {
            let b_first = b_first.next().unwrap();
            if c > b_first {
                return Ordering::Greater;
            }
            if b_first > c {
                return Ordering::Less;
            }
        }
        Ordering::Equal
    }
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.cards.strength > other.cards.strength {
            return Some(Ordering::Greater);
        }
        if self.cards.strength < other.cards.strength {
            return Some(Ordering::Less);
        }

        let mut a_first = self.cards.raw_cards.iter();
        let mut b_first = other.cards.raw_cards.iter();
        while let Some(c) = a_first.next() {
            let b_first = b_first.next().unwrap();
            if c > b_first {
                return Some(Ordering::Greater);
            }
            if b_first > c {
                return Some(Ordering::Less);
            }
        }
        Some(Ordering::Equal)
    }
}

type RawCards = Vec<Card>;

#[derive(Debug, Eq, PartialEq)]
struct Cards {
    strength: Strength,
    raw_cards: RawCards,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    JPartTwo,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl Into<Card> for char {
    fn into(self) -> Card {
        match self {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::T,
            'J' => unsafe {
                return match &RULES {
                    Some(Rule::PartOne) => Card::J,
                    Some(Rule::PartTwo) => Card::JPartTwo,
                    None => unreachable!("There should always be a rule set!"),
                };
            },
            'Q' => Card::Q,
            'K' => Card::K,
            'A' => Card::A,
            _ => unreachable!("Invalid card found!"),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
enum Strength {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Into<Strength> for HashMap<char, u8> {
    fn into(self) -> Strength {
        let rule = unsafe { &RULES.clone() };
        let mut modifier = &0;
        if self.get_key_value(&'J').is_some() && rule == &Some(Rule::PartTwo) {
            modifier = self.get(&'J').unwrap();
        }
        let mut sorted = self.iter().collect::<Vec<_>>();
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
}

#[derive(PartialEq, Debug, Clone)]
enum Rule {
    PartOne,
    PartTwo,
}

static mut RULES: Option<Rule> = None;

fn sort_and_fold(mut games: Vec<Game>) -> usize {
    games.sort();

    games
        .iter()
        .enumerate()
        .fold(0, |acc, (i, g)| (g.bid * (i + 1)) + acc)
}

fn main() -> Result<()> {
    unsafe {
        RULES = Some(Rule::PartOne);
    };
    let games = parse_lines::<Game>("data/seven.input".into())?;

    println!("Part One: {}", sort_and_fold(games));

    unsafe {
        RULES = Some(Rule::PartTwo);
    };
    let games = parse_lines::<Game>("data/seven.input".into())?;

    println!("Part Two: {}", sort_and_fold(games));

    Ok(())
}
