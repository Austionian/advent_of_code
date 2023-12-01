use std::fs;

enum Instruction {
    Add,
    Multiply,
    Halt,
    Error,
}

fn get_instruction(n: usize) -> Instruction {
    match n {
        1 => Instruction::Add,
        2 => Instruction::Multiply,
        99 => Instruction::Halt,
        _ => Instruction::Error,
    }
}

fn parse_program(v: &mut Vec<usize>) -> anyhow::Result<()> {
    let mut cursor = 0;

    while cursor < v.len() {
        let instruction = get_instruction(v[cursor]);

        let first_val = v[cursor + 1];
        let second_val = v[cursor + 2];
        let update_at = v[cursor + 3];

        match instruction {
            Instruction::Add => {
                v[update_at] = v[first_val] + v[second_val];
                cursor += 4;
            }
            Instruction::Multiply => {
                v[update_at] = v[first_val] * v[second_val];
                cursor += 4;
            }
            Instruction::Halt => break,
            Instruction::Error => break,
        }
    }

    Ok(())
}

fn main() -> anyhow::Result<()> {
    let mut program = fs::read_to_string("data/day2.input")?
        .trim()
        .split(',')
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    program[1] = 12;
    program[2] = 2;

    parse_program(&mut program)?;

    println!("Part One: {}", program[0]);

    let program = fs::read_to_string("data/day2.input")?
        .trim()
        .split(',')
        .filter_map(|n| n.parse::<usize>().ok())
        .collect::<Vec<_>>();

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program_clone = program.clone();
            program_clone[1] = noun;
            program_clone[2] = verb;

            parse_program(&mut program_clone)?;

            if program_clone[0] == 19690720 {
                println!("Part Two: {}", 100 * noun + verb);
                return Ok(());
            }
        }
    }

    Ok(())
}
