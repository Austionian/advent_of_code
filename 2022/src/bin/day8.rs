use anyhow::Result;

// PART TWO
fn main() -> Result<()> {
    let col = include_str!("./day8_input.txt")
        .lines()
        .map(|x| {
            x.chars()
                .map(|y| y.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let height = &col.len();
    let width = &col[0].len();

    let mut scores: Vec<usize> = vec![];

    for (i, x) in col.iter().enumerate() {
        for (j, y) in x.iter().enumerate() {
            if i == 0 || j == 0 || i == height - 1 || j == width - 1 {
                continue;
            }
            let mut up = 0;
            let mut down = 0;
            let mut left = 0;
            let mut right = 0;
            {
                let mut m = i.clone();
                while m > 0 {
                    up += 1;
                    m -= 1;
                    // Look Up
                    if col[m][j] >= *y {
                        break;
                    }
                }
                for n in (i + 1)..*height {
                    down += 1;
                    // Look Down
                    if col[n][j] >= *y {
                        break;
                    }
                }
                let mut o = j.clone();
                while o > 0 {
                    left += 1;
                    o -= 1;
                    // Look Left
                    if col[i][o] >= *y {
                        break;
                    }
                }
                for q in (j + 1)..*width {
                    right += 1;
                    // Look Right
                    if col[i][q] >= *y {
                        break;
                    }
                }
            }
            scores.push(up * down * left * right);
        }
    }

    scores.sort_by(|a, b| b.cmp(a));

    println!("{}", scores[0]);

    Ok(())
}

// PART ONE
// fn main() -> Result<()> {
//     let col = include_str!("./day8_test_input.txt")
//         .lines()
//         .map(|x| {
//             x.chars()
//                 .map(|y| y.to_digit(10).unwrap())
//                 .collect::<Vec<_>>()
//         })
//         .collect::<Vec<_>>();
//
//     let height = &col.len();
//     let width = &col[0].len();
//
//     let mut total = 0;
//
//     for (i, x) in col.iter().enumerate() {
//         for (j, y) in x.iter().enumerate() {
//             if i == 0 || j == 0 || i == height - 1 || j == width - 1 {
//                 continue;
//             }
//             let mut total_modified = false;
//             {
//                 for m in 0..i {
//                     if !total_modified {
//                         // Look Up
//                         if col[m][j] >= *y {
//                             break;
//                         }
//                         // if it gets to here that means nothing above it was taller.
//                         if m == i - 1 {
//                             total += 1;
//                             total_modified = true;
//                         }
//                     }
//                 }
//                 for n in (i + 1)..*height {
//                     if !total_modified {
//                         // Look Down
//                         if col[n][j] >= *y {
//                             break;
//                         }
//                         if n == *height - 1 {
//                             total += 1;
//                             total_modified = true;
//                         }
//                     }
//                 }
//                 for o in 0..j {
//                     if !total_modified {
//                         // Look Left
//                         if col[i][o] >= *y {
//                             break;
//                         }
//                         if o == j - 1 {
//                             total += 1;
//                             total_modified = true;
//                         }
//                     }
//                 }
//                 for q in (j + 1)..*width {
//                     if !total_modified {
//                         // Look Right
//                         if col[i][q] >= *y {
//                             break;
//                         }
//                         if q == *width - 1 {
//                             total += 1;
//                         }
//                     }
//                 }
//             }
//         }
//     }
//
//     let outside_trees = height * 2 + ((width - 2) * 2);
//     println!("{}", outside_trees + total);
//
//     Ok(())
// }
