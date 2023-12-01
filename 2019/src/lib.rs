use anyhow::Result;
use std::str::FromStr;

pub fn parse_from_string<T: FromStr>(path: &str) -> Result<Vec<T>> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn parse_and_transform<T: FromStr + std::iter::Sum>(path: &str, f: fn(T) -> T) -> Result<T> {
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .map(f)
        .sum::<T>())
}
