use std::io::prelude::*;
use std::io::BufReader;
use std::str::FromStr;

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    // Only checks whether other is fully contained by self.
    fn full_overlap(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    // Checks if there's any overlap in either direction (does not require 2 calls).
    fn any_overlap(&self, other: &Range) -> bool {
        (self.start < other.start && other.start <= self.end) || (self.start >= other.start && self.start <= other.end)
    }

}

impl FromStr for Range {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        let start = parts[0].parse().unwrap();
        let end = parts[1].parse().unwrap();
        Ok(Range { start, end })
    }
}

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| {
            l.expect("line")
        })
        .collect();

    let mut full_overlap = 0;
    let mut any_overlap = 0;

    for line in lines {
        let parts: Vec<&str> = line.split(',').collect();
        let range1: Range = parts[0].parse().unwrap();
        let range2: Range = parts[1].parse().unwrap();

        if range1.full_overlap(&range2) || range2.full_overlap(&range1) {
            full_overlap += 1;
        }

        if range1.any_overlap(&range2) {
            any_overlap += 1;
        }
    }

    println!("full overlap count: {:?}", full_overlap);

    println!("any overlap count: {:?}", any_overlap);

    Ok(())
}
