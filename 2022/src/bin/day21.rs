use anyhow::Result;
use std::collections::HashMap;

fn find_value(name: &str, coll: &HashMap<&str, &str>) -> isize {
    let value = coll.get(name);

    match value {
        Some(v) => {
            let num = v.parse::<isize>();
            match num {
                Ok(x) => x,
                _ => {
                    let (left, right, op) = parse_line(v);

                    let left = find_value(left, coll);
                    let right = find_value(right, coll);

                    if name == "root" {
                        println!("{}, {}", left, right);
                        return left - right;
                    }
                    match op {
                        "*" => left * right,
                        "+" => left + right,
                        "-" => left - right,
                        "/" => left / right,
                        _ => panic!("thanks advent of code!"),
                    }
                }
            }
        }
        _ => {
            let (left, right, op) = parse_line(name);

            let left = find_value(left, coll);
            let right = find_value(right, coll);

            match op {
                "*" => left * right,
                "+" => left + right,
                "-" => left - right,
                "/" => left / right,
                _ => panic!("thanks advent of code!"),
            }
        }
    }
}

fn parse_line(line: &str) -> (&str, &str, &str) {
    let vs = line.split(' ').collect::<Vec<&str>>();
    (vs[0], vs[2], vs[1])
}

fn main() -> Result<()> {
    let coll: HashMap<_, _> = include_str!("./day21_test_input.txt")
        .lines()
        .map(|line| line.split_once(": ").unwrap())
        .collect();

    let result = find_value("humn", &coll);

    println!("{}", result);

    Ok(())
}

// PART ONE
// fn find_value(name: &str, coll: &HashMap<&str, &str>) -> usize {
//     let value = coll.get(name);
//
//     match value {
//         Some(v) => {
//             let num = v.parse::<usize>();
//             match num {
//                 Ok(x) => x,
//                 _ => {
//                     let (left, right, op) = parse_line(v);
//
//                     let left = find_value(left, coll);
//                     let right = find_value(right, coll);
//
//                     match op {
//                         "*" => left * right,
//                         "+" => left + right,
//                         "-" => left - right,
//                         "/" => left / right,
//                         _ => panic!("thanks advent of code!"),
//                     }
//                 }
//             }
//         }
//         _ => {
//             let (left, right, op) = parse_line(name);
//
//             let left = find_value(left, coll);
//             let right = find_value(right, coll);
//
//             match op {
//                 "*" => left * right,
//                 "+" => left + right,
//                 "-" => left - right,
//                 "/" => left / right,
//                 _ => panic!("thanks advent of code!"),
//             }
//         }
//     }
// }
//
// fn parse_line(line: &str) -> (&str, &str, &str) {
//     let vs = line.split(' ').collect::<Vec<&str>>();
//     (vs[0], vs[2], vs[1])
// }
//
// fn main() -> Result<()> {
//     let coll: HashMap<&str, &str> = include_str!("./day21_input.txt")
//         .lines()
//         .map(|line| line.split_once(": ").unwrap())
//         .collect();
//
//     let result = find_value("root", &coll);
//
//     println!("{}", result);
//
//     Ok(())
// }
