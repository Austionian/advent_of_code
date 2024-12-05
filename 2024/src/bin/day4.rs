fn is_xmas(slice: &[char]) -> bool {
    slice == &['X', 'M', 'A', 'S'] || slice == &['S', 'A', 'M', 'X']
}

fn char_check(line: &Vec<char>) -> usize {
    line.windows(4)
        .fold(0, |acc, slice| if is_xmas(slice) { acc + 1 } else { acc })
}

fn get_vertical_slices(input: &str, width: usize) -> Vec<Vec<char>> {
    let mut vertical_slices = Vec::new();

    for _ in 0..width {
        vertical_slices.push(Vec::new());
    }

    input.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, ch)| {
            vertical_slices[i].push(ch);
        });
    });

    vertical_slices
}

fn find_next(
    chars: &Vec<Vec<char>>,
    ch: char,
    x: usize,
    y: usize,
    check: (isize, isize),
    height: usize,
    width: usize,
    mul: isize,
) -> bool {
    let new_x = x as isize + check.0 * mul;
    let new_y = y as isize + check.1 * mul;

    if new_x < 0 || new_x >= width as isize {
        return false;
    }
    if new_y < 0 || new_y >= height as isize {
        return false;
    }

    chars[new_x as usize][new_y as usize] == ch
}

fn part_one(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().fold(0, |acc, _| acc + 1);

    let vertical_slices = get_vertical_slices(input, width);

    input.lines().fold(0, |acc, line| {
        acc + char_check(&line.chars().collect::<Vec<_>>())
    }) + vertical_slices
        .iter()
        .fold(0, |acc, line| acc + char_check(line))
        + get_diagonals(input, width, height)
}

fn get_diagonals(input: &str, width: usize, height: usize) -> usize {
    let mut found = 0;
    let chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let checks = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    for i in 0..width {
        for j in 0..height {
            if chars[i][j] == 'X' {
                for check in checks {
                    if find_next(&chars, 'M', i, j, check, height, width, 1) {
                        if find_next(&chars, 'A', i, j, check, height, width, 2) {
                            if find_next(&chars, 'S', i, j, check, height, width, 3) {
                                found += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    found
}

fn find_x(
    chars: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    check: (isize, isize),
    height: usize,
    width: usize,
) -> Option<char> {
    let new_x = x as isize + check.0;
    let new_y = y as isize + check.1;

    if new_x < 0 || new_x >= width as isize {
        return None;
    }
    if new_y < 0 || new_y >= height as isize {
        return None;
    }

    Some(chars[new_x as usize][new_y as usize])
}

fn get_xes(input: &str, width: usize, height: usize) -> usize {
    let mut found = 0;
    let chars = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let checks = [(1, 1), (1, -1), (-1, 1), (-1, -1)];
    for i in 0..width {
        for j in 0..height {
            if chars[i][j] == 'A' {
                let mut m = Vec::new();
                let mut s = Vec::new();

                for check in checks {
                    match find_x(&chars, i, j, check, height, width) {
                        Some('M') => m.push(check),
                        Some('S') => s.push(check),
                        _ => break,
                    }
                }

                if m.len() == 2 && s.len() == 2 {
                    if m.iter().fold(0, |acc, tup| acc + tup.0 + tup.1) != 0 {
                        found += 1;
                    }
                }
            }
        }
    }

    found
}

fn part_two(input: &str) -> usize {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().fold(0, |acc, _| acc + 1);

    get_xes(input, width, height)
}

fn main() {
    println!(
        "Part one: {}",
        part_one(include_str!("../../data/day4.txt"))
    );
    println!(
        "Part two: {}",
        part_two(include_str!("../../data/day4.txt"))
    );
}
