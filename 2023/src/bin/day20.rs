use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Debug, Eq, PartialEq)]
enum Prefix {
    FlipFlop,
    Conjunction,
    Start,
}

#[derive(Debug)]
struct Node {
    prefix: Prefix,
    key: String,
    destinations: Vec<String>,
    // Only relevent to Flip-Flop nodes
    on: bool,
    // Only relevent to Conjunction nodes
    // bool is low_pulse
    inputs: Option<HashMap<String, bool>>,
}

#[derive(Debug)]
struct Pulse {
    origin: String,
    destination: String,
    low_pulse: bool,
}

impl FromStr for Node {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (key, rest) = s.split_once(' ').unwrap();
        let (_, destinations) = rest.split_once(' ').unwrap();
        let destinations = destinations.split(", ").map(|s| s.to_string()).collect();

        if key.contains('%') {
            let key = key.strip_prefix("%").unwrap().to_string();
            Ok(Node {
                prefix: Prefix::FlipFlop,
                key,
                destinations,
                on: false,
                inputs: None,
            })
        } else if key.contains("&") {
            let key = key.strip_prefix("&").unwrap().to_string();
            Ok(Node {
                prefix: Prefix::Conjunction,
                key,
                destinations,
                on: false,
                inputs: None,
            })
        } else {
            Ok(Node {
                prefix: Prefix::Start,
                key: key.to_string(),
                destinations,
                on: false,
                inputs: None,
            })
        }
    }
}

fn main() {
    let mut address_book = HashMap::new();

    include_str!("../../data/twenty.input")
        .lines()
        .for_each(|line| {
            let node = line.parse::<Node>().unwrap();

            address_book.insert(node.key.to_string(), node);
        });

    let conjunctions = address_book
        .iter()
        .filter(|(_, v)| v.prefix == Prefix::Conjunction)
        .map(|(k, _)| k.clone())
        .collect::<Vec<_>>();

    for conjunction in conjunctions.iter() {
        let mut inputs = HashMap::new();
        address_book.iter().for_each(|(k, v)| {
            v.destinations.iter().for_each(|destination| {
                if destination == conjunction {
                    inputs.insert(k.clone(), false);
                }
            })
        });

        address_book
            .entry(conjunction.to_string())
            .and_modify(|v| v.inputs = Some(inputs));
    }

    let mut high = 0;
    let mut low = 0;

    for _ in 0..1_000 {
        // The button being pressed.
        low += 1;
        let start = address_book.get("broadcaster").unwrap();
        let mut pulses = start
            .destinations
            .iter()
            .map(|destination| Pulse {
                origin: "broadcaster".to_string(),
                destination: destination.to_string(),
                low_pulse: true,
            })
            .collect::<VecDeque<_>>();

        while pulses.len() > 0 {
            let pulse = pulses.pop_front().unwrap();

            if pulse.low_pulse {
                low += 1;
            } else {
                high += 1;
            }

            if pulse.destination == "output" {
                continue;
            }

            let node = if let Some(node) = address_book.get_mut(pulse.destination.as_str()) {
                node
            } else {
                continue;
            };

            match node.prefix {
                Prefix::FlipFlop => {
                    if pulse.low_pulse {
                        if node.on {
                            for destination in node.destinations.iter() {
                                pulses.push_back(Pulse {
                                    origin: node.key.to_string(),
                                    destination: destination.to_string(),
                                    low_pulse: true,
                                });
                            }
                        } else {
                            for destination in node.destinations.iter() {
                                pulses.push_back(Pulse {
                                    origin: node.key.to_string(),
                                    destination: destination.to_string(),
                                    low_pulse: false,
                                });
                            }
                        }
                        node.on = !node.on
                    }
                }
                Prefix::Conjunction => {
                    node.inputs.as_mut().map(|inputs| {
                        inputs
                            .entry(pulse.origin)
                            .and_modify(|v| *v = !pulse.low_pulse);
                        let all_high = inputs.iter().fold(true, |acc, (_, v)| acc && *v);
                        // Manually found the four origin conjunctions to rx and found their LCM.
                        if all_high {
                            for destination in node.destinations.iter() {
                                pulses.push_back(Pulse {
                                    origin: node.key.to_string(),
                                    destination: destination.to_string(),
                                    low_pulse: true,
                                });
                            }
                        } else {
                            for destination in node.destinations.iter() {
                                pulses.push_back(Pulse {
                                    origin: node.key.to_string(),
                                    destination: destination.to_string(),
                                    low_pulse: false,
                                });
                            }
                        }
                    });
                }
                _ => unreachable!("Invalid prefix match in pulse."),
            }
        }
    }
    println!("Part One: {}", high * low);
}
