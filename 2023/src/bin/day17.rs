use anyhow::{anyhow, Result};
use pathfinding::prelude::dijkstra;

#[derive(Copy, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos(u32, u32);

impl Pos {
    fn successors(&self, weights: &Vec<Vec<usize>>, height: u32, width: u32) -> Vec<(Pos, usize)> {
        let &Pos(x, y) = self;
        if x == 0 && y == 0 {
            return vec![Pos(x + 1, y), Pos(x, y + 1)]
                .into_iter()
                .map(|p| (p, weights[p.0 as usize][p.1 as usize]))
                .collect();
        }
        if x == 0 && y == height - 1 {
            return vec![Pos(x + 1, y), Pos(x, y - 1)]
                .into_iter()
                .map(|p| (p, weights[p.0 as usize][p.1 as usize]))
                .collect();
        }
        if x == width - 1 && y == height - 1 {
            return vec![Pos(x - 1, y), Pos(x, y - 1)]
                .into_iter()
                .map(|p| (p, weights[p.0 as usize][p.1 as usize]))
                .collect();
        }
        if x == width - 1 && y == 0 {
            return vec![Pos(x - 1, y), Pos(x, y + 1)]
                .into_iter()
                .map(|p| (p, weights[p.0 as usize][p.1 as usize]))
                .collect();
        }
        if x > 0 && y == 0 && x < width {
            return vec![Pos(x - 1, y), Pos(x + 1, y), Pos(x, y + 1)]
                .into_iter()
                .map(|p| (p, weights[p.0 as usize][p.1 as usize]))
                .collect();
        }
        if x == width - 1 && y > 0 && y < height {
            return vec![Pos(x - 1, y), Pos(x, y + 1), Pos(x, y - 1)]
                .into_iter()
                .map(|p| (p, weights[p.0 as usize][p.1 as usize]))
                .collect();
        }
        if x == 0 && y > 0 && y < height {
            return vec![Pos(x + 1, y), Pos(x, y - 1), Pos(x, y + 1)]
                .into_iter()
                .map(|p| (p, weights[p.0 as usize][p.1 as usize]))
                .collect();
        }
        if y == height - 1 && x > 0 && x < width {
            return vec![Pos(x + 1, y), Pos(x - 1, y), Pos(x, y - 1)]
                .into_iter()
                .map(|p| (p, weights[p.0 as usize][p.1 as usize]))
                .collect();
        }
        if x > 0 && y > 0 && x < width && y < height {
            return vec![Pos(x, y + 1), Pos(x, y - 1), Pos(x + 1, y), Pos(x - 1, y)]
                .into_iter()
                .map(|p| (p, weights[p.0 as usize][p.1 as usize]))
                .collect();
        }

        unreachable!("Uncaught next case");
    }
}

fn main() -> Result<()> {
    let weights = include_str!("../../data/seventeen.test")
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|v| v.to_digit(10))
                .map(|v| v as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = weights.len() as u32;
    let width = weights[0].len() as u32;

    let goal: Pos = Pos(width - 1, height - 1);
    let result = dijkstra(
        &Pos(0, 0),
        |p| p.successors(&weights, height, width),
        |p| *p == goal,
    )
    .ok_or(anyhow!("No path found!"))?;

    println!("{:?}", result);

    Ok(())
}
