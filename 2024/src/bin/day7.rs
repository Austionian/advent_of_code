const TEST: &'static str = "7290: 6 8 6 15";

fn part_one() -> usize {
    include_str!("../../data/day7.txt")
        .lines()
        .fold(0, |acc, line| {
            let (answer, values) = line.split_once(": ").unwrap();
            let answer = answer.parse::<usize>().unwrap();

            let values = values
                .split_whitespace()
                .filter_map(|value| value.parse::<usize>().ok())
                .collect::<Vec<_>>();

            let mut prev = Vec::new();
            for v in values.iter() {
                let mut temp = Vec::new();
                if prev.is_empty() {
                    prev.push(*v);
                    continue;
                } else {
                    for pre in prev.iter() {
                        temp.push(*v * *pre);
                        temp.push(*v + *pre);
                    }
                }
                prev = temp;
            }

            let mut out = acc;
            for v in prev.iter() {
                if *v == answer {
                    out += answer;
                    break;
                }
            }

            out
        })
}

fn part_two() -> usize {
    include_str!("../../data/day7.txt")
        .lines()
        .fold(0, |acc, line| {
            let (answer, values) = line.split_once(": ").unwrap();
            let answer = answer.parse::<usize>().unwrap();

            let values = values
                .split_whitespace()
                .filter_map(|value| value.parse::<usize>().ok())
                .collect::<Vec<_>>();

            let mut prev = Vec::new();
            for v in values.iter() {
                let mut temp = Vec::new();
                if prev.is_empty() {
                    prev.push(*v);
                    continue;
                } else {
                    for pre in prev.iter() {
                        temp.push(*v * *pre);
                        temp.push(*v + *pre);

                        let mut new_con = String::new();
                        new_con.push_str(&pre.to_string());
                        new_con.push_str(&v.to_string());
                        temp.push(new_con.parse().unwrap());
                    }
                }
                prev = temp;
            }

            let mut out = acc;
            for v in prev.iter() {
                if *v == answer {
                    out += answer;
                    break;
                }
            }

            out
        })
}

fn main() {
    println!("Part one: {}", part_one());
    println!("Part two: {}", part_two());
}
