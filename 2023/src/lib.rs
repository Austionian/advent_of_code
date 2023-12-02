use std::{fs, path::PathBuf, str::FromStr};

pub fn parse_lines<T>(path: PathBuf) -> anyhow::Result<Vec<T>>
where
    T: FromStr,
{
    Ok(fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .collect())
}

pub fn parse_and_transform<T, R>(path: PathBuf, f: fn(T) -> R) -> anyhow::Result<R>
where
    T: FromStr,
    R: std::iter::Sum,
{
    Ok(fs::read_to_string(path)?
        .lines()
        .filter_map(|line| line.parse::<T>().ok())
        .map(f)
        .sum())
}
