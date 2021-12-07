use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let mut reader = Box::new(BufReader::new(stdin));
    let mut line: String = String::new();
    reader.read_line(&mut line)?;
    let positions: Vec<i32> = line.trim().split(',').map(|v| v.parse::<i32>().unwrap()).collect();

    println!("{:?}", positions);


    let (min_cost_position, min_cost) = find_min_cost(&positions);

    println!("min_cost_position: {}, min_cost: {}", min_cost_position, min_cost);

    Ok(())
}

fn find_min_cost(positions: &Vec<i32>) -> (i32, i32) {
    let max_position = positions.iter().max().expect("expecting max");
    let mut min_cost = i32::max_value();
    let mut min_cost_position = i32::max_value();
    for pos in 0..*max_position {
        let cost = positions.iter().map(|p| (p - pos).abs()).sum();
        if cost < min_cost {
            min_cost = cost;
            min_cost_position = pos;
        }
    }

    (min_cost_position, min_cost)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_min_cost_for_example() {
        let positions = vec![16,1,2,0,4,2,7,1,2,14];

        assert_eq!(find_min_cost(&positions), (2, 37));
    }
}
