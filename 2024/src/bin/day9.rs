//const TEST: &'static str = "2333133121414131402";

fn part_one(input: &str) -> usize {
    let mut inp = input.trim().to_owned();
    if inp.len() % 2 != 0 {
        inp.push('0');
    }

    let blocks = inp
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, data)| {
            let [file_size, free_space] = data else {
                unreachable!("we made the input even!");
            };

            let mut new = Vec::new();
            for _ in 0..file_size.to_digit(10).unwrap() {
                new.push(Some(id));
            }

            for _ in 0..free_space.to_digit(10).unwrap() {
                new.push(None);
            }

            new
        })
        .collect::<Vec<_>>();

    let mut output = Vec::new();
    let mut ring_pointer = 0;

    for (i, ch) in blocks.iter().enumerate() {
        if i >= blocks.len() - ring_pointer {
            break;
        }
        if let Some(v) = ch {
            output.push(*v);
        } else {
            while blocks.iter().nth_back(ring_pointer).unwrap().is_none() {
                ring_pointer += 1;
            }
            let v = blocks.iter().nth_back(ring_pointer).unwrap().unwrap();
            output.push(v);
            ring_pointer += 1;
        }
    }

    output
        .iter()
        .enumerate()
        .fold(0, |acc, (i, ch)| acc + *ch * i)
}

#[derive(Clone, Debug)]
struct Mem {
    value: Option<usize>,
    len: u32,
}

fn part_two(input: &str) -> usize {
    let mut inp = input.trim().to_owned();
    if inp.len() % 2 != 0 {
        inp.push('0');
    }

    let blocks = inp
        .chars()
        .collect::<Vec<_>>()
        .chunks(2)
        .enumerate()
        .flat_map(|(id, data)| {
            let [file_size, free_space] = data else {
                unreachable!("we made the input even!");
            };

            vec![
                Mem {
                    value: Some(id),
                    len: file_size.to_digit(10).unwrap(),
                },
                Mem {
                    value: None,
                    len: free_space.to_digit(10).unwrap(),
                },
            ]
        })
        .collect::<Vec<_>>();

    let b = blocks
        .iter()
        .filter_map(|bl| if bl.len > 0 { Some(bl.clone()) } else { None })
        .collect::<Vec<_>>();

    let mut output = b.clone();

    for mem in b.iter().rev() {
        let cur = output.iter().position(|m| m.value == mem.value).unwrap();
        if mem.value.is_none() {
            continue;
        }
        let to_rm = output.iter().position(|m| m.value == mem.value).unwrap();
        let len = mem.len;
        let mut to_update_i = 0;
        let mut update = None;
        for (i, m) in output.iter_mut().enumerate() {
            if m.value.is_none() && m.len >= len {
                update = Some(m);
                to_update_i = i;
                break;
            }
        }

        if cur < to_update_i {
            continue;
        }

        if let Some(m) = update {
            if m.len == mem.len {
                m.value = mem.value;

                if let Some(v) = output.get_mut(to_rm) {
                    v.value = None;
                }
            } else {
                let update_and_dup = output
                    .iter()
                    .position(|mem| mem.value.is_none() && mem.len >= len);
                if let Some(pointer) = update_and_dup {
                    let updated = output.get_mut(pointer).unwrap();
                    // leftover
                    let diff = updated.len - mem.len;

                    updated.len = mem.len;
                    updated.value = mem.value;

                    output.insert(
                        pointer + 1,
                        Mem {
                            value: None,
                            len: diff,
                        },
                    );

                    if let Some(v) = output.get_mut(to_rm + 1) {
                        v.value = None;
                    }
                }
            }
        }
    }

    let mut pos = 0;

    output.iter().fold(0, |acc, mem| {
        let mut out = 0;
        for _ in 0..mem.len {
            out += pos * mem.value.unwrap_or(0);
            pos += 1;
        }
        out + acc
    })
}

fn main() {
    println!(
        "Part one: {}",
        part_one(include_str!("../../data/day9.txt"))
    );
    println!(
        "Part two: {}",
        part_two(include_str!("../../data/day9.txt"))
    );
}
