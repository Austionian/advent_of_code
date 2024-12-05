use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

struct Rule {
    x: usize,
    y: usize,
}

struct Update {
    u: Vec<usize>,
}

impl FromStr for Rule {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once("|").unwrap();
        Ok(Rule {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        })
    }
}

impl FromStr for Update {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Update {
            u: s.split(",").map(|v| v.parse().unwrap()).collect(),
        })
    }
}

fn parse_map_and_updates() -> (HashMap<usize, Vec<usize>>, &'static str) {
    let (rules, updates) = include_str!("../../data/day5.txt")
        .split_once("\n\n")
        .unwrap();
    let mut map: HashMap<usize, Vec<usize>> = HashMap::new();

    rules
        .lines()
        .filter_map(|line| line.parse::<Rule>().ok())
        .for_each(|rule| {
            map.entry(rule.x)
                .and_modify(|v| v.push(rule.y))
                .or_insert(vec![rule.y]);
        });

    (map, updates)
}

fn is_valid(map: &HashMap<usize, Vec<usize>>, update: &Vec<usize>) -> bool {
    let mut key = None;
    let mut valid = true;

    for v in update.iter() {
        if key.is_none() {
            key = Some(v);
            continue;
        } else {
            if let Some(rule) = map.get(key.unwrap()) {
                if rule.contains(v) {
                    key = Some(v);
                    continue;
                } else {
                    valid = false;
                    break;
                }
            } else {
                valid = false;
                break;
            }
        }
    }

    valid
}

fn find_next(map: &HashMap<usize, Vec<usize>>, key: &Option<&usize>, value: usize) -> usize {
    if let Some(rule) = map.get(&key.unwrap()) {
        if rule.contains(&value) {
            return *key.unwrap();
        }
    }
    find_next(map, &Some(&value), *key.unwrap())
}

fn fix(update: &Vec<usize>, map: &HashMap<usize, Vec<usize>>) -> Vec<usize> {
    if is_valid(map, &update) {
        return update.clone();
    };

    let mut key = None;
    let mut new = VecDeque::new();

    for (i, v) in update.iter().enumerate() {
        if key.is_none() {
            key = Some(v);
            new.push_back(*v);
            continue;
        } else {
            // if correctly ordered
            if let Some(rule) = map.get(key.unwrap()) {
                if rule.contains(v) {
                    key = Some(v);
                    new.push_back(*v);
                    continue;
                }
            }
            new.insert(i - 1, find_next(map, &key, *v));
        }
    }

    fix(&new.into(), map)
}

fn part_one() -> usize {
    let (map, updates) = parse_map_and_updates();

    updates
        .lines()
        .filter_map(|line| line.parse::<Update>().ok())
        .fold(0, |acc, update| {
            if is_valid(&map, &update.u) {
                acc + *update
                    .u
                    .get((update.u.len() as f32 / 2.0) as usize)
                    .unwrap()
            } else {
                acc
            }
        })
}

fn part_two() -> usize {
    let (map, updates) = parse_map_and_updates();

    updates
        .lines()
        .filter_map(|line| line.parse::<Update>().ok())
        .fold(0, |acc, update| {
            if !is_valid(&map, &update.u) {
                acc + *fix(&update.u, &map)
                    .get((update.u.len() as f32 / 2.0) as usize)
                    .unwrap()
            } else {
                acc
            }
        })
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
