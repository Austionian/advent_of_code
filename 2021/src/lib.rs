use anyhow::Result;
use std::str::FromStr;

/// Function that parse every line in a txt file to a
/// given generic.
pub fn read_one_at_a_time<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn split_on_comma<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .split(',')
        .filter_map(|v| v.trim().parse().ok())
        .collect())
}

pub fn two_d_vec<T>(path: &str) -> Result<Vec<Vec<T>>>
where
    T: FromStr,
    Vec<T>: FromIterator<u32>,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .map(|line| line.trim().chars().filter_map(|c| c.to_digit(10)).collect())
        .collect())
}

pub fn cardinal_directions(
    x: usize,
    y: usize,
    x_bound: usize,
    y_bound: usize,
) -> Vec<(usize, usize)> {
    let mut dirs = Vec::new();

    if let Some(x) = x.checked_sub(1) {
        dirs.push((x, y));
    }
    if let Some(y) = y.checked_sub(1) {
        dirs.push((x, y));
    }
    if let Some(x) = x.checked_add(1) {
        if x < x_bound {
            dirs.push((x, y));
        }
    }
    if let Some(y) = y.checked_add(1) {
        if y < y_bound {
            dirs.push((x, y));
        }
    }

    dirs
}
