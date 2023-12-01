#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(PartialEq, Debug)]
enum PlayType {
    Rock,
    Paper,
    Sissors,
}

fn parse_line(play: &str, outcome: &str) -> (PlayType, Outcome) {
    (parse_play(play), parse_outcome(outcome))
}

fn parse_play(play: &str) -> PlayType {
    if play == "A" {
        return PlayType::Rock;
    }
    if play == "B" {
        return PlayType::Paper;
    }
    return PlayType::Sissors;
}

fn parse_outcome(outcome: &str) -> Outcome {
    if outcome == "Y" {
        return Outcome::Draw;
    }
    if outcome == "X" {
        return Outcome::Loss;
    }
    return Outcome::Win;
}

fn win<'a>(theirs: &PlayType, outcome: &'a Outcome) -> (&'a Outcome, PlayType) {
    let play = match outcome {
        Outcome::Win => match theirs {
            PlayType::Rock => PlayType::Paper,
            PlayType::Paper => PlayType::Sissors,
            PlayType::Sissors => PlayType::Rock,
        },
        Outcome::Loss => match theirs {
            PlayType::Rock => PlayType::Sissors,
            PlayType::Paper => PlayType::Rock,
            PlayType::Sissors => PlayType::Paper,
        },
        Outcome::Draw => match theirs {
            PlayType::Paper => PlayType::Paper,
            PlayType::Sissors => PlayType::Sissors,
            PlayType::Rock => PlayType::Rock,
        },
    };
    return (outcome, play);
}

fn totals(outcome: &Outcome, playtype: &PlayType) -> i32 {
    let point1 = match outcome {
        Outcome::Win => 6,
        Outcome::Loss => 0,
        Outcome::Draw => 3,
    };
    let point2 = match playtype {
        PlayType::Rock => 1,
        PlayType::Paper => 2,
        PlayType::Sissors => 3,
    };
    point1 + point2
}

fn main() {
    let collection: Vec<(&str, &str)> = include_str!("./day2_input.txt")
        .lines()
        .map(|x| {
            let mut it = x.split(' ');
            (it.next().unwrap(), it.next().unwrap())
        })
        .collect();
    let plays: Vec<(PlayType, Outcome)> =
        collection.iter().map(|(x, y)| parse_line(x, y)).collect();
    let outcomes: Vec<(&Outcome, PlayType)> = plays.iter().map(move |(x, y)| win(x, y)).collect();
    let values: Vec<i32> = outcomes.iter().map(|(x, y)| totals(x, y)).collect();
    let answer: i32 = values.iter().sum();
    println!("{:?}", answer);
}
