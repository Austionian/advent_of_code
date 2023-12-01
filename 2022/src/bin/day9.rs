use anyhow::Result;
use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() -> Result<()> {
    let input = include_str!("./day9_input.txt")
        .lines()
        .map(|line| {
            let (dir, amount) = line.split_once(' ').unwrap();

            (dir, amount.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut seen_points = HashSet::new();
    let mut knots = vec![];

    for _ in 0..9 {
        knots.push(Point { x: 0, y: 0 })
    }

    // Always include the origin
    seen_points.insert(Point { x: 0, y: 0 });

    for (dir, amt) in input.iter() {
        for (i, knot) in knots.iter_mut().enumerate() {
            // println!("evaluating dir: {} amt: {}", dir, amt);
            if dir == &"R" {
                for _ in 0..*amt {
                    knot.x += 1;
                    if knots[i + 1].x + 1 < knot.x {
                        if knot.y - knots[i + 1].y != 0 {
                            knots[i + 1].x += 1;
                            knots[i + 1].y = knot.y;
                        } else {
                            knots[i + 1].x += 1;
                        }
                        seen_points.insert(knots[i + 1]);
                        // println!("Adding R {:?}", knots[i + 1]);
                    }
                }
            }
            if dir == &"L" {
                for _ in 0..*amt {
                    knot.x -= 1;
                    let cur_dif = knot.x - knots[i + 1].x;
                    if cur_dif.abs() > 1 {
                        if knot.y - knots[i + 1].y != 0 {
                            knots[i + 1].x -= 1;
                            knots[i + 1].y = knot.y;
                        } else {
                            knots[i + 1].x -= 1;
                        }
                        // println!("Adding L {:?}", knots[i + 1]);
                        seen_points.insert(knots[i + 1]);
                    }
                }
            }
            if dir == &"U" {
                for _ in 0..*amt {
                    knot.y += 1;
                    if knots[i + 1].y + 1 < knot.y {
                        if knot.x - knots[i + 1].x != 0 {
                            knots[i + 1].y += 1;
                            knots[i + 1].x = knot.x;
                        } else {
                            knots[i + 1].y += 1;
                        }
                        // println!("Adding U {:?}", knots[i + 1]);
                        seen_points.insert(knots[i + 1]);
                    }
                }
            }
            if dir == &"D" {
                for _ in 0..*amt {
                    knot.y -= 1;
                    let curr_dif = knot.y - knots[i + 1].y;
                    if curr_dif.abs() > 1 {
                        if knot.x - knots[i + 1].x != 0 {
                            knots[i + 1].y -= 1;
                            knots[i + 1].x = knot.x;
                        } else {
                            knots[i + 1].y -= 1;
                        }
                        // println!("Adding D {:?}", knots[i + 1]);
                        seen_points.insert(knots[i + 1]);
                    }
                }
            }
        }
    }

    println!("{:?}", seen_points.len());

    Ok(())
}
