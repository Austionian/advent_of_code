use anyhow::Result;

// PART TWO
fn main() -> Result<()> {
    let mut stack = vec![(0, "")];
    let mut valid_sizes: Vec<usize> = vec![];
    let unused = 25_725_669;
    let min_needed = 30_000_000 - unused;

    let input = include_str!("./day7_input.txt")
        .split('\n')
        .collect::<Vec<&str>>();

    for x in input.iter() {
        let v1 = x.split(' ').nth(0).unwrap_or("");
        let v2 = x.split(' ').nth(1).unwrap_or("");
        let v3 = x.split(' ').nth(2).unwrap_or("");
        let len = stack.len();
        // if v1 is dir then it's a directory to add to the hashmap
        if v1 == "dir" || v2 == "ls" {
            continue;
        }
        if v1 == "$" && v2 == "cd" && v3 != ".." {
            stack.push((0, v3));
        }
        if v1 == "$" && v2 == "cd" && v3 == ".." {
            let (v, _) = stack.pop().unwrap();
            if v >= min_needed {
                valid_sizes.push(v);
            }
            stack[len - 2] = (stack.last().unwrap().0 + v, "");
        }
        if let Ok(i) = v1.parse::<usize>() {
            stack[len - 1] = (stack.last().unwrap().0 + i, "");
        }
    }

    valid_sizes.sort();
    println!("{:?}", valid_sizes[0]);

    Ok(())
}

// PART ONE
// fn main() -> Result<()> {
//     let mut stack = vec![(0, "")];
//     let mut tot = 0;
//     let needed = 30_000_000;
//     let used = 44_274_331;
//
//     let min_needed = used - needed;
//
//     let input = include_str!("./day7_input.txt")
//         .split('\n')
//         .collect::<Vec<&str>>();
//
//     for x in input.iter() {
//         let v1 = x.split(' ').nth(0).unwrap_or("");
//         let v2 = x.split(' ').nth(1).unwrap_or("");
//         let v3 = x.split(' ').nth(2).unwrap_or("");
//         let len = stack.len();
//         // if v1 is dir then it's a directory to add to the hashmap
//         if v1 == "dir" || v2 == "ls" {
//             continue;
//         }
//         if v1 == "$" && v2 == "cd" && v3 != ".." {
//             stack.push((0, v3));
//         }
//         if v1 == "$" && v2 == "cd" && v3 == ".." {
//             let (v, _) = stack.pop().unwrap();
//             if v <= 100_000 {
//                 tot += v;
//             }
//             stack[len - 2] = (stack.last().unwrap().0 + v, "");
//         }
//         if let Ok(i) = v1.parse::<usize>() {
//             stack[len - 1] = (stack.last().unwrap().0 + i, "");
//         }
//     }
//
//     println!("{:?}", tot);
//
//     Ok(())
// }
