use std::io::prelude::*;
use std::io::BufReader;
use std::collections::BTreeSet;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| {
            l.expect("line")
        })
        .collect();

    let priorities: Vec<u32> = lines.iter()
        .map(|l| {
            let items: Vec<char> = l.chars().collect();
            let comp1: BTreeSet<&char> = BTreeSet::from_iter(&items[0..items.len()/2]);
            let comp2: BTreeSet<&char> = BTreeSet::from_iter(&items[items.len()/2..]);

            let mut intersection = comp1.intersection(&comp2);
            // assert_eq!(intersection.collect::<Vec<&&char>>().len(), 1);

            intersection.next().map(|c| (**c as u8) as char)
        })
        .flatten()
        .map(get_priority)
        .collect();

    println!("sum of priorities: {:?}", priorities.iter().sum::<u32>());

    let group_priority_sum = lines[..].chunks(3).map(|group: &[String]| {
        let intersection: &BTreeSet<char> = &group[..].iter()
            .map(|l| BTreeSet::from_iter(l.chars()))
            .reduce(|acc, item| {
                acc.intersection(&item).map(|c| *c).collect::<BTreeSet<char>>()
            }).unwrap();
        assert_eq!(intersection.len(), 1);
        get_priority(*intersection.iter().next().unwrap())
    }).sum::<u32>();

    println!("group priority sum: {:?}", group_priority_sum);


    Ok(())
}

fn get_priority(p: char) -> u32 {
    if p.is_lowercase() {
        p as u32 - 'a' as u32 + 1
    } else if p.is_uppercase() {
        p as u32 - 'A' as u32 + 27
    } else {
        panic!("unexpected priority!");
    }
}
