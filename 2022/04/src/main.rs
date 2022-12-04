use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeSet;
use std::str::FromStr;

struct Range {
    start: u32,
    end: u32,
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

        let r1 = BTreeSet::from_iter((range1.start)..=(range1.end));
        let r2 = BTreeSet::from_iter((range2.start)..=(range2.end));
        if r1.difference(&r2).collect::<BTreeSet<&u32>>().is_empty() || r2.difference(&r1).collect::<BTreeSet<&u32>>().is_empty() {
            full_overlap += 1;
        }

        if r1.intersection(&r2).next().is_some() {
            any_overlap += 1;
        }

    }

    println!("full overlap count: {:?}", full_overlap);

    println!("any overlap count: {:?}", any_overlap);

    Ok(())
}
