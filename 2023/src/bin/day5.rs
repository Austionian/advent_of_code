use anyhow::anyhow;

#[derive(Debug)]
struct Map {
    ranges: Vec<Range>,
}

#[derive(Debug)]
struct Range {
    output_low: usize,
    output_high: usize,
    input_low: usize,
}

fn get_prev(out: usize, map: &Map) -> usize {
    for range in &map.ranges {
        if out >= range.output_low && out <= range.output_high {
            return (out - range.output_low) + range.input_low;
        }
    }
    out
}

fn is_a_starting_seed(seed: usize, seeds: &Vec<(usize, usize)>) -> bool {
    for (start, end) in seeds.iter() {
        if seed >= *start && seed <= *end {
            return true;
        }
    }
    false
}

fn part_one(seeds: &Vec<usize>, rest: &str) -> anyhow::Result<usize> {
    let mut locations = Vec::new();
    for seed in seeds {
        let mut value = seed.clone();
        let rest = rest.split("\n\n");
        for maps in rest {
            let mut maps = maps.lines();
            maps.next().unwrap();
            for map in maps {
                let ranges = map
                    .split_whitespace()
                    .filter_map(|v| v.parse::<usize>().ok())
                    .collect::<Vec<_>>();

                if (ranges[1]..(ranges[1] + ranges[2])).contains(&value) {
                    value = (value - ranges[1]) + ranges[0];
                    break;
                }
            }
        }
        locations.push(value);
    }

    Ok(*locations
        .iter()
        .min()
        .ok_or(anyhow!("No min value found!"))?)
}

fn part_two(seeds: &Vec<usize>, rest: &str) -> anyhow::Result<usize> {
    let mut parsed_seeds = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        parsed_seeds.push((seeds[i], seeds[i] + seeds[i + 1]));
        i += 2;
    }

    let maps = rest
        .split("\n\n")
        .map(|group| {
            let mut ranges = Vec::new();
            let mut group = group.lines();
            group.next().unwrap(); // throw away the title
            for map in group {
                let values = map
                    .split_whitespace()
                    .filter_map(|n| n.parse::<usize>().ok())
                    .collect::<Vec<_>>();
                ranges.push(Range {
                    output_low: values[0],
                    output_high: values[0] + values[2],
                    input_low: values[1],
                })
            }
            Map { ranges }
        })
        .collect::<Vec<Map>>();

    let mut low = 0;
    loop {
        let mut start = low;
        for map in maps.iter().rev() {
            start = get_prev(start, map);
        }

        if is_a_starting_seed(start, &parsed_seeds) {
            break;
        }

        low += 1;
    }

    Ok(low)
}

fn main() -> anyhow::Result<()> {
    let (seeds, rest) = include_str!("../../data/five.input")
        .split_once("\n\n")
        .ok_or(anyhow!("No seeds found!"))?;

    let seeds = seeds
        .split_once(":")
        .ok_or(anyhow!("No colon found!"))?
        .1
        .trim()
        .split_whitespace()
        .filter_map(|v| v.parse::<usize>().ok())
        .collect::<Vec<_>>();

    println!("Part One: {}", part_one(&seeds, &rest)?);

    println!("Part Two: {}", part_two(&seeds, &rest)?);

    Ok(())
}
