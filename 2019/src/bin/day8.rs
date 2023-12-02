use std::fs;

const HEIGHT: usize = 6;
const LENGTH: usize = 25;
const AREA: usize = HEIGHT * LENGTH;

fn parse_layer(s: &str) -> Vec<u32> {
    s.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn main() -> anyhow::Result<()> {
    let image = fs::read_to_string("data/day8.input")?;
    let mut cursor = 0;
    let mut layers = Vec::new();
    while cursor + AREA < image.len() {
        layers.push(parse_layer(&image[cursor..cursor + AREA]));
        cursor += AREA;
    }

    let zeros = layers
        .iter()
        .enumerate()
        .map(|(i, layer)| {
            let count = layer.iter().fold(0, |acc, n| {
                if n == &0 {
                    return acc + 1;
                }
                acc
            });
            (i, count)
        })
        .min_by(|x, y| x.1.cmp(&y.1))
        .ok_or(anyhow::anyhow!("No max tuple found!"))?;

    let mut ones = 0;
    let mut twos = 0;

    layers[zeros.0].iter().for_each(|n| {
        if n == &1 {
            ones += 1;
        }
        if n == &2 {
            twos += 1;
        }
    });

    println!("Part One: {}", ones * twos);

    let mut final_image = Vec::new();

    let mut cursor = 0;
    while cursor < AREA {
        for layer in &layers {
            if layer[cursor] == 2 {
                continue;
            }
            final_image.push(layer[cursor]);
            break;
        }
        cursor += 1;
    }

    let mut cursor = 0;

    println!("Part Two:");
    while cursor < final_image.len() {
        let line = &final_image[cursor..cursor + LENGTH]
            .iter()
            .map(|n| n.to_string())
            .collect::<String>();
        println!("{line}");
        cursor += LENGTH;
    }

    Ok(())
}
