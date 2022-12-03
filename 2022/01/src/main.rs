use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let mut calories: Vec<i32> = reader
        .lines()
        .map(|l| {
            l.expect("line")
        })
        .fold(vec![0], |mut acc: Vec<i32>, l: String| {
            if l == "" {
                acc.push(0);
            } else {
                if let Some(last) = acc.last_mut() {
                    *last = *last + l.parse::<i32>().expect("not a number");
                }
            }

            acc
        });

    calories.sort();
    println!("most calories: {:?}", calories.last());

    println!("sum of top 3: {:?}", calories[calories.len() - 3..].iter().sum::<i32>());


    Ok(())
}
