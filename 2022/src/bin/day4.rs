use anyhow::Result;

fn parse_vec(line: Vec<&str>) -> Vec<Vec<usize>> {
    line.iter()
        .map(|x| {
            x.split('-')
                .flat_map(str::parse::<usize>)
                .collect::<Vec<usize>>()
        })
        .collect()
}

// fn com_range(range: Vec<usize>) -> usize {
//     let it = range.iter();
//     let v1 = range.next().unwrap();
//     let v2 = range.next().unwrap();
// }

// start_val >= start_val2 && end_val >= end_val2
//
fn comp(line: &mut Vec<Vec<usize>>) -> usize {
    let mut it = line.iter_mut();

    let group1: &mut Vec<usize> = it.next().unwrap();
    let group2: &mut Vec<usize> = it.next().unwrap();

    // Part 1
    // if (group1.iter().min().unwrap() <= group2.iter().min().unwrap()
    //     && group1.iter().max().unwrap() >= group2.iter().max().unwrap())
    //     || (group2.iter().min().unwrap() <= group1.iter().min().unwrap()
    //         && group2.iter().max().unwrap() >= group1.iter().max().unwrap())

    // Part 2
    if (group1.iter().max().unwrap() < group2.iter().min().unwrap())
        || (group2.iter().max().unwrap() < group1.iter().min().unwrap())
    {
        return 0;
    }
    1
}

fn main() -> Result<()> {
    let input: Vec<_> = include_str!("./day4_input.txt")
        .lines()
        .map(|x| x.split(',').collect::<Vec<&str>>())
        .collect();

    let mut parsed: Vec<Vec<Vec<usize>>> =
        input.iter().map(move |x| parse_vec(x.to_vec())).collect();

    let tots = parsed
        .iter_mut()
        .map(move |x| return comp(x))
        .sum::<usize>();

    println!("{:?}", tots);

    Ok(())
}
