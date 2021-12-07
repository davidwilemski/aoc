use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let mut reader = Box::new(BufReader::new(stdin));
    let mut line: String = String::new();
    reader.read_line(&mut line)?;
    let fish: Vec<Lanternfish> = line.trim().split(',').map(|v| Lanternfish(v.parse::<i32>().unwrap())).collect();

    let fish_count = simulate_lanternfish(fish.clone(), 80);

    println!("{} fish after 80 days", fish_count);

    let school = collapse_school(&fish);

    println!("{} fish after 256 days", simulate_lanternfish_school(school, 256));

    Ok(())
}

fn simulate_lanternfish(mut fish: Vec<Lanternfish>, days: i32) -> i64 {
    for day in 0..days {
        let mut fish_to_add = 0;
        for f in fish.as_mut_slice() {
            if f.simulate() {
                fish_to_add += 1;
            }
        }

        for _ in 0..fish_to_add {
            fish.push(Lanternfish::new());
        }

        println!("after day {} there are {} fish", day, fish.len());
    }

    fish.len() as i64
}

fn simulate_lanternfish_school(mut school: HashMap<i32, i64>, days: i32) -> i64 {
    for day in 0..days {
        let mut new_school = HashMap::new();
        for (timer, count) in school.iter() {
            println!("day {}, timer {}, count {}", day, timer, count);
            if *timer == 0 {
                let fish_count = new_school.entry(6).or_insert(0);
                *fish_count += *count;
                new_school.entry(8).or_insert(*count);
            } else {
                let fish_count = new_school.entry(*timer - 1).or_insert(0);
                *fish_count += *count;
            }
        }
        school = new_school;
    }

    school.values().sum()
}

// build a hash map of timer -> count of fish
fn collapse_school(fish: &Vec<Lanternfish>) -> HashMap<i32, i64> {
    let mut school = HashMap::new();
    for f in fish {
        let count = school.entry(f.0).or_insert(0);
        *count += 1;
    }

    school
}

#[derive(Debug, Clone)]
struct Lanternfish(i32);

impl Lanternfish {
    fn new() -> Lanternfish {
        Lanternfish(8)
    }

    fn simulate(&mut self) -> bool {
        if self.0 > 0 {
            self.0 -= 1;
            false
        } else {
            self.0 = 6;
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_subtracts_one_from_timer_and_returns_false() {
        let mut fish = Lanternfish(4);

        assert_eq!(fish.simulate(), false);
        assert_eq!(fish.0, 3);
    }

    #[test]
    fn test_simulate_resets_timer_to_six_and_returns_true() {
        let mut fish = Lanternfish(0);

        assert_eq!(fish.simulate(), true);
        assert_eq!(fish.0, 6);
    }

    #[test]
    fn test_simulate_lanternfish_with_example() {
        let fish: Vec<Lanternfish> = vec![3,4,3,1,2].drain(..).map(|v| Lanternfish(v)).collect();
        assert_eq!(simulate_lanternfish(fish, 18), 26);

        let fish: Vec<Lanternfish> = vec![3,4,3,1,2].drain(..).map(|v| Lanternfish(v)).collect();
        assert_eq!(simulate_lanternfish(fish, 80), 5934);

    }

    #[test]
    fn test_simulate_lanternfish_school_with_example() {
        let fish: Vec<Lanternfish> = vec![3,4,3,1,2].drain(..).map(|v| Lanternfish(v)).collect();
        assert_eq!(simulate_lanternfish_school(collapse_school(&fish), 18), 26);

        let fish: Vec<Lanternfish> = vec![3,4,3,1,2].drain(..).map(|v| Lanternfish(v)).collect();
        assert_eq!(simulate_lanternfish_school(collapse_school(&fish), 80), 5934);

        let fish: Vec<Lanternfish> = vec![3,4,3,1,2].drain(..).map(|v| Lanternfish(v)).collect();
        assert_eq!(simulate_lanternfish_school(collapse_school(&fish), 256), 26984457539);
    }
}
