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
