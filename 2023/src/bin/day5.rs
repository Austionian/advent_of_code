use anyhow::anyhow;

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

    let mut locations = Vec::new();
    for seed in seeds {
        let mut value = seed;
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

    let res = locations
        .iter()
        .min()
        .ok_or(anyhow!("No min value found!"))?;

    println!("Part One: {}", res);

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

    let mut parsed_seeds = Vec::new();
    let mut i = 0;
    while i < seeds.len() {
        let mut v = (seeds[i]..seeds[i] + seeds[i + 1]).collect::<Vec<usize>>();
        parsed_seeds.append(&mut v);
        i += 2;
    }

    let mut locations = Vec::new();
    for seed in parsed_seeds {
        let mut value = seed;
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

    let res = locations
        .iter()
        .min()
        .ok_or(anyhow!("No min value found!"))?;

    println!("Part Two: {res}");

    Ok(())
}
