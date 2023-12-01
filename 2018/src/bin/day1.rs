use anyhow::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    // PART 1
    // let res: i32 = include_str!("./day1_input.txt")
    //     .lines()
    //     .map(|x| x.parse::<i32>().unwrap())
    //     .sum();
    // println!("{res}");

    let input = include_str!("./day1_input.txt")
        .lines()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    let mut set = HashSet::new();
    let mut running_total = 0;
    let mut end: Option<i32> = None;
    set.insert(running_total);

    while end == None {
        for v in input.iter() {
            running_total += v;
            let res = set.insert(running_total);
            if !res {
                end = Some(running_total);
                println!("{running_total}");
                break;
            }
        }
    }

    Ok(())
}
