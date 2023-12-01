use anyhow::Result;

#[derive(Debug)]
enum InstructionType {
    Noop,
    Addx,
}

#[derive(Debug)]
struct Instruction {
    instruction_type: InstructionType,
    value: i32,
}

struct Cycle {
    current: u32,
    value: i32,
}

fn main() -> Result<()> {
    let input = include_str!("./day10_input.txt")
        .lines()
        .map(|line| {
            let (dir, v) = line.split_once(' ').unwrap_or(("noop", "0"));
            Instruction {
                instruction_type: match dir {
                    "addx" => InstructionType::Addx,
                    "noop" => InstructionType::Noop,
                    _ => panic!("No way jose!"),
                },
                value: v.parse::<i32>().unwrap_or(0),
            }
        })
        .collect::<Vec<_>>();

    let mut cycle = Cycle {
        current: 1,
        value: 1,
    };
    let mut output = String::new();

    for instruction in input.iter() {
        match instruction.instruction_type {
            InstructionType::Noop => {
                if cycle.value == (cycle.current as i32)
                    || cycle.value == (cycle.current as i32) - 1
                    || cycle.value + 1 == (cycle.current as i32) - 1
                {
                    output.push('#');
                } else {
                    output.push('.');
                }
                if (cycle.current) % 40 == 0 {
                    output.push('\n');
                    cycle.current = 0;
                }
                cycle.current += 1;
            }
            InstructionType::Addx => {
                for _ in 0..2 {
                    if cycle.value == (cycle.current as i32)
                        || cycle.value == (cycle.current as i32) - 1
                        || cycle.value + 1 == (cycle.current as i32) - 1
                    {
                        output.push('#');
                    } else {
                        output.push('.');
                    }
                    if (cycle.current) % 40 == 0 {
                        output.push('\n');
                        cycle.current = 0;
                    }
                    cycle.current += 1;
                }
                cycle.value += instruction.value;
            }
        }
    }

    println!("{}", output);

    Ok(())
}
