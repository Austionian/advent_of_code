#![feature(map_try_insert)]
use anyhow::Result;
use std::collections::HashMap;

const LEN: usize = 14;

fn find_start(mes: &str, starting: usize) -> Option<usize> {
    let mut col = HashMap::new();

    for c in mes.chars() {
        if let Err(_) = col.try_insert(c, 1) {
            return None;
        }
    }

    Some(starting + LEN)
}

fn main() -> Result<()> {
    let input = include_str!("./day6_input.txt").chars().collect::<Vec<char>>();

    for (i, _c) in input.iter().enumerate() {
        let mut mes = String::from("");
        for x in i..i + LEN {
            mes.push(*input.iter().nth(x).unwrap());
        }
        if let Some(v) = find_start(&mes, i) {
            println!("{}", v);
            return Ok(());
        }
    }

    Ok(())
}
