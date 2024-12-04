fn mul_parser() -> usize {
    let mut do_check = true;
    include_str!("../../data/day3.txt")
        .split("mul")
        .skip(1)
        .fold(0, |acc, values| {
            let mut iter = values.chars();
            let mut x = String::new();
            let mut y = String::new();
            let mut good = true;
            let mut comma = false;
            let mut ending_paren = false;
            if do_check {
                if let Some('(') = iter.next() {
                    for _ in 0..3 {
                        if let Some(ch) = iter.next() {
                            match ch {
                                '0'..'9' => x.push_str(&ch.to_string()),
                                '9' => x.push_str("9"),
                                ',' => {
                                    comma = true;
                                    break;
                                }
                                _ => good = false,
                            }
                        }
                    }

                    if !comma && iter.next().unwrap_or(' ') != ',' {
                        good = false;
                    }

                    if good {
                        for _ in 0..3 {
                            if let Some(ch) = iter.next() {
                                match ch {
                                    '0'..'9' => y.push_str(&ch.to_string()),
                                    '9' => y.push_str("9"),
                                    ')' => {
                                        ending_paren = true;
                                        break;
                                    }
                                    // it's no good any longer
                                    _ => {
                                        y = 0.to_string();
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    if !ending_paren && iter.next().unwrap_or(' ') != ')' {
                        y = 0.to_string();
                    }
                }
            }
            let mut substr = values;
            if values.contains("don't()") {
                while substr.contains("don't()") {
                    let dont = substr.find("don't()").unwrap();
                    substr = &substr[dont + 7..];
                }
                do_check = substr.contains("do()")
            } else if !do_check {
                do_check = values.contains("do()")
            }
            acc + x.parse::<usize>().unwrap_or(0) * y.parse::<usize>().unwrap_or(0)
        })
}

fn main() {
    println!("Part two: {}", mul_parser());
}
