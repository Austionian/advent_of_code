use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    default: String,
}

#[derive(Debug)]
struct Rule {
    value: Value,
    amount: isize,
    order: Order,
    destination: String,
}

#[derive(Debug)]
enum Value {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Order {
    Greater,
    Less,
}

impl FromStr for Value {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Value::X,
            "m" => Value::M,
            "a" => Value::A,
            "s" => Value::S,
            _ => unreachable!("invalid value given."),
        })
    }
}

struct Part {
    values: Vec<isize>,
}

impl FromStr for Part {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.strip_prefix('{').unwrap().strip_suffix('}').unwrap();
        let mut p = Vec::new();

        let (v, rest) = s.split_once(',').unwrap();
        let (_, v) = v.split_once('=').unwrap();

        p.push(v.parse::<isize>().unwrap());

        let (v, rest) = rest.split_once(',').unwrap();
        let (_, v) = v.split_once('=').unwrap();

        p.push(v.parse::<isize>().unwrap());

        let (v, rest) = rest.split_once(',').unwrap();
        let (_, v) = v.split_once('=').unwrap();

        p.push(v.parse::<isize>().unwrap());

        let (_, v) = rest.split_once('=').unwrap();

        p.push(v.parse::<isize>().unwrap());

        Ok(Part { values: p })
    }
}

fn main() {
    let (map, parts) = include_str!("../../data/nineteen.input")
        .split_once("\n\n")
        .unwrap();

    let mut workflow = HashMap::new();

    map.lines().for_each(|line| {
        let (key, rest) = line.strip_suffix("}").unwrap().split_once('{').unwrap();

        let mut rules_iter = rest.split(',');

        let mut default = String::new();
        let mut rules = Vec::new();

        while let Some(rule) = rules_iter.next() {
            if let Some((rule, destination)) = rule.split_once(':') {
                if rule.contains(">") {
                    let (value, amount) = rule.split_once('>').unwrap();
                    rules.push(Rule {
                        value: value.parse().unwrap(),
                        amount: amount.parse().unwrap(),
                        order: Order::Greater,
                        destination: destination.to_string(),
                    })
                } else {
                    let (value, amount) = rule.split_once('<').unwrap();
                    rules.push(Rule {
                        value: value.parse().unwrap(),
                        amount: amount.parse().unwrap(),
                        order: Order::Less,
                        destination: destination.to_string(),
                    })
                }
            } else {
                default = rule.to_string();
            }
        }

        workflow.insert(key.to_string(), Workflow { rules, default });
    });

    let res = parts
        .lines()
        .map(|line| {
            let part = line.parse::<Part>().unwrap();

            let mut key = "in";
            'outer: while key != "A" && key != "R" {
                let w = workflow.get(key).unwrap();

                for rule in &w.rules {
                    let v = match rule.value {
                        Value::X => part.values[0],
                        Value::M => part.values[1],
                        Value::A => part.values[2],
                        Value::S => part.values[3],
                    };
                    match rule.order {
                        Order::Less => {
                            if v < rule.amount {
                                key = &rule.destination;
                                continue 'outer;
                            }
                        }
                        Order::Greater => {
                            if v > rule.amount {
                                key = &rule.destination;
                                continue 'outer;
                            }
                        }
                    }
                }
                key = &w.default;
            }

            if key == "A" {
                return part.values.iter().sum();
            }
            0
        })
        .sum::<isize>();

    println!("{:?}", res);
}
