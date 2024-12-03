use regex::Regex;
use std::fmt;
use rayon::prelude::*;

#[derive(Debug)]
pub struct ParsedLine {
    left: u64,
    right: u64,
}

impl fmt::Display for ParsedLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "left: {}, right: {}",
            self.left, self.right
        )
    }
}


pub fn read_text_file() -> Vec<ParsedLine> {
    let parse_regex = Regex::new(r"^(.*?)\s+(.*?)\s*?$").unwrap();

    include_str!("../resources/sample.txt")
        .lines()
        .filter_map(|line| {
            parse_regex.captures(line).and_then(|captures| {
                let left = captures.get(1)?.as_str().parse::<u64>().ok()?;
                let right = captures.get(2)?.as_str().parse::<u64>().ok()?;

                Some(ParsedLine {
                    left,
                    right,
                })
            })
        })
        .collect()
}

pub fn solve_day() -> u64 {
    let rows = read_text_file();
    let (mut lefts, mut rights): (Vec<u64>, Vec<u64>) = rows
        .into_par_iter() // Parallel iterator
        .map(|line| (line.left, line.right))
        .unzip(); // Parallel unzip
    
    // lefts.sort();
    // rights.sort();

    // Use par_iter and zip to parallelize the computation
    let results: u64 = lefts
        .par_iter()
        .map(|&item| {
            // Count how many times `item` appears in `vec2`
            item * rights.par_iter().filter(|&&x| x == item).count() as u64
        })
        .reduce(|| 0, |acc, val| acc + val);

    println!("{:?}", results);
    results
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

    #[test]
    fn solve_day_test() {
        solve_day();
    }
}
