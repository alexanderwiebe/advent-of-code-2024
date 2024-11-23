use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub struct ParsedLine {
    prefix: u64,
    value: String,
    suffix: u64,
}

impl fmt::Display for ParsedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "prefix: {}, value: {}, suffix: {}",
            self.prefix, self.value, self.suffix
        )
    }
}

pub fn read_text_file() -> Vec<ParsedLine> {
    let parse_regex = Regex::new(r"^(.*?) (.*?) (.*?)$").unwrap();

    include_str!("../resources/problem.txt")
        .lines()
        // .filter(|line| Regex::new(r"^.*?$").unwrap().is_match(line))
        .filter_map(|line| {
            parse_regex.captures(line).and_then(|captures| {
                // Try to parse the prefix and suffix safely
                let prefix = captures.get(1)?.as_str().parse::<u64>().ok()?;
                let value = captures.get(2)?.as_str().to_string();
                let suffix = captures.get(3)?.as_str().parse::<u64>().ok()?;

                Some(ParsedLine {
                    prefix,
                    value,
                    suffix,
                })
            })
        }) // Convert matching lines into ParsedLine objects
        .collect()
}

pub fn solve_day() -> u64 {
    let rows = read_text_file();
    rows.iter()
        .map(|line| line.prefix + line.suffix)
        .fold(0, |acc, row| acc + row)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
