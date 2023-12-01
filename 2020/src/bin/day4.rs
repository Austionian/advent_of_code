use anyhow::Result;
use aoc2020::one_separated_by_empty_line;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

const ECL: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

#[derive(Debug)]
struct Passport {
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

fn get_owned_string(input: Option<&String>) -> Option<String> {
    match input {
        Some(s) => Some(s.to_string()),
        None => None,
    }
}

impl FromStr for Passport {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values = s.split(|c| c == ' ' || c == '\n').collect::<Vec<_>>();

        let mut map = HashMap::new();

        let _ = values
            .iter()
            .map(|pair| {
                let (field, value) = pair
                    .split_once(':')
                    .unwrap_or(("no semicolon found.", "error"));

                map.insert(field.clone().to_owned(), value.clone().to_owned());
            })
            .collect::<Vec<_>>();

        let byr = map.get("byr");
        let iyr = map.get("iyr");
        let eyr = map.get("eyr");
        let hgt = map.get("hgt");
        let hcl = map.get("hcl");
        let ecl = map.get("ecl");
        let pid = map.get("pid");

        Ok(Passport {
            byr: get_owned_string(byr),
            iyr: get_owned_string(iyr),
            eyr: get_owned_string(eyr),
            hgt: get_owned_string(hgt),
            hcl: get_owned_string(hcl),
            ecl: get_owned_string(ecl),
            pid: get_owned_string(pid),
        })
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_extra_valid(&self) -> bool {
        self.valid_ecl()
            && self.valid_iyr()
            && self.valid_eyr()
            && self.valid_hgt()
            && self.valid_hcl()
            && self.valid_pid()
            && self.valid_byr()
    }

    fn valid_byr(&self) -> bool {
        if let Some(n) = &self.byr {
            let n = n.parse::<u32>().expect("invalid byr");
            return n >= 1920 && n <= 2002;
        }
        false
    }

    fn valid_iyr(&self) -> bool {
        if let Some(n) = &self.iyr {
            let n = n.parse::<u32>().expect("invalid iyr");
            return n >= 2010 && n <= 2020;
        }
        false
    }

    fn valid_eyr(&self) -> bool {
        if let Some(n) = &self.eyr {
            let n = n.parse::<u32>().expect("invalid eyr");
            return n >= 2020 && n <= 2030;
        }
        false
    }

    fn valid_hgt(&self) -> bool {
        if let Some(v) = &self.hgt {
            if v.contains("cm") {
                let n = v.split("cm").next().unwrap();
                let n = n.parse::<u32>().unwrap_or(0);
                return n >= 150 && n <= 193;
            }
            if v.contains("in") {
                let n = v.split("in").next().unwrap();
                let n = n.parse::<u32>().unwrap_or(0);
                return n >= 59 && n <= 76;
            }
        }
        false
    }

    fn valid_hcl(&self) -> bool {
        if let Some(v) = &self.hcl {
            let re = Regex::new(r"#[0-9a-f]{6}").unwrap();
            return re.is_match(v);
        }
        false
    }

    fn valid_ecl(&self) -> bool {
        if let Some(v) = &self.ecl {
            return ECL.contains(&v.as_str());
        }
        false
    }

    fn valid_pid(&self) -> bool {
        if let Some(n) = &self.pid {
            let re = Regex::new(r"[0-9]{9}").unwrap();
            return re.is_match(n);
        }
        false
    }
}

fn part_one(path: &str) -> Result<usize> {
    Ok(one_separated_by_empty_line::<Passport>(path)?
        .iter()
        .filter(|passport| passport.is_valid())
        .count())
}

// off by one for some reason
fn part_two(path: &str) -> Result<usize> {
    Ok(one_separated_by_empty_line::<Passport>(path)?
        .iter()
        .filter(|passport| passport.is_extra_valid())
        .count())
}

fn main() -> Result<()> {
    println!("Part One: {}", part_one("./data/4.input")?);
    println!("Part Two: {}", part_two("./data/4.input")?);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one_test() {
        let res = part_one("./data/4.test").unwrap();
        assert_eq!(res, 2);
    }

    #[test]
    fn part_two_test() {
        let res = part_two("./data/4.alt_test").unwrap();
        assert_eq!(res, 4);
    }
}
