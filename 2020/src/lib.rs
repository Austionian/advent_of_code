use anyhow::Result;
use std::collections::HashSet;
use std::str::FromStr;

pub fn one_at_a_time<T>(path: &str) -> Result<HashSet<T>>
where
    T: FromStr + std::hash::Hash + std::cmp::Eq,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn one_per_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn one_separated_by_empty_line<T>(path: &str) -> Result<Vec<T>>
where
    T: FromStr,
{
    Ok(std::fs::read_to_string(path)?
        .split("\n\n")
        .filter_map(|item| item.parse::<T>().ok())
        .collect())
}
