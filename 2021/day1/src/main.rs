use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let values: Vec<i32> = reader.lines().map(|v| v.unwrap().parse::<i32>().unwrap()).collect();

    let increasing = map_increasing(&values);

    println!("increasing count: {}", increasing.iter().filter(|v| **v).count());

    println!(
        "sliding window increasing count: {}",
        map_increasing(&sliding_window_sum(3, &values)).iter().filter(|v| **v).count()
    );

    Ok(())
}

// Given a slice of integers, return a Vec of boolean values indicating whether each integer was
// an increase over the previous value.
fn map_increasing(values: &[i32]) -> Vec<bool> {
    values.iter()
        .enumerate()
        .map(|(idx, val)| {
            if idx == 0 {
                false
            } else {
                val > &values[idx -1]
            }
        })
        .collect()
}

fn sliding_window_sum(window: usize, values: &[i32]) -> Vec<i32> {
    values.windows(window).map(|w| w.iter().sum()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_increasing_example() {
        let values = vec![
            199, // Not increasing because no prior values
            200, // true
            208, // true
            210, // true
            200, // false
            207, // true
            240, // true
            269, // true
            260, // false
            263, // true
        ];

        assert_eq!(map_increasing(&values), vec![false, true, true, true, false, true, true, true, false, true])
    }

    #[test]
    fn sliding_window_sum_example() {
        let values = vec![
            199,
            200,
            208,
            210,
            200,
            207,
            240,
            269,
            260,
            263,
        ];

        assert_eq!(sliding_window_sum(3, &values), vec![607, 618, 618, 617, 647, 716, 769, 792])
    }
}
