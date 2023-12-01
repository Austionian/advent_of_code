use anyhow::Result;

fn main() -> Result<()> {
    let (top, bottom) = get_top_and_bottom(include_str!("./day5_input.txt"));

    let coll = top
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let mut crates = vec![];

    for (j, z) in coll.iter().rev().enumerate() {
        for (i, v) in z.chunks(4).enumerate() {
            if j == 0 {
                crates.push(vec![]);
                continue;
            }
            if v[1] != ' ' {
                crates[i].push(v[1]);
            }
        }
    }

    let _dir = bottom
        .lines()
        .map(|x| {
            let (mo, fr, to) = get_directions(x);

            let mut new_stack = vec![];
            for _ in 0..mo {
                let popped = crates[fr - 1].pop().unwrap();
                new_stack.push(popped);
            }
            new_stack.reverse();
            crates[to - 1].append(&mut new_stack);
        })
        .collect::<Vec<_>>();

    let mut ans = String::new();

    for stacks in crates {
        let len = stacks.len();
        ans.push_str(&stacks[len - 1].to_string())
    }

    println!("answer: {}", ans);
    Ok(())
}

fn get_top_and_bottom(input: &str) -> (&str, &str) {
    input.split_once("\n\n").unwrap()
}

fn get_directions(line: &str) -> (usize, usize, usize) {
    (
        line.split(' ').nth(1).unwrap().parse::<usize>().unwrap(),
        line.split(' ').nth(3).unwrap().parse::<usize>().unwrap(),
        line.split(' ').nth(5).unwrap().parse::<usize>().unwrap(),
    )
}
