use std::io::prelude::*;
use std::io::BufReader;

#[derive(Debug)]
enum Instruction {
    Forward(i32),
    Up(i32),
    Down(i32),
}

impl PartialEq for Instruction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Instruction::Forward(v1), Instruction::Forward(v2)) => v1 == v2,
            (Instruction::Up(v1), Instruction::Up(v2)) => v1 == v2,
            (Instruction::Down(v1), Instruction::Down(v2)) => v1 == v2,
            _ => false

        }
    }
}

impl Eq for Instruction {}

#[derive(Debug)]
struct EndPosition {
    horizontal: i32,
    vertical: i32,
    aim: i32,
}

impl PartialEq for EndPosition {
    fn eq(&self, other: &Self) -> bool {
        self.horizontal == other.horizontal && self.vertical == other.vertical
    }
}

impl Eq for EndPosition {}

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = Box::new(BufReader::new(stdin));
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    let instructions: Vec<Instruction> = read_instructions(&lines);

    let end_pos = find_end_position(&instructions);

    println!("{:?}", end_pos);

    Ok(())
}

fn read_instructions(lines: &[String]) -> Vec<Instruction> {
    lines.iter().map(|line| {
        let split: Vec<&str> = line.split(' ').collect();
        match split[..] {
            [direction, amount] => {
                let amount_int = amount.parse::<i32>()
                    .expect("Instruction amount not able to be parsed");
                match direction {
                    "up" => Instruction::Up(amount_int),
                    "down" => Instruction::Down(amount_int),
                    "forward" => Instruction::Forward(amount_int),
                    &_ => panic!("unexpected instruction direction")
                }

            },
            _ => panic!("Split resulted in too many values")
        }

    })
    .collect()
}

fn find_end_position(instructions: &[Instruction]) -> EndPosition {
    let initial = EndPosition {horizontal: 0, vertical: 0, aim: 0};
    instructions.iter().fold(initial, |acc, instr| {
        match instr {
            Instruction::Forward(v) => EndPosition { horizontal: acc.horizontal + v, vertical: acc.vertical + (v * acc.aim), ..acc },
            Instruction::Up(v) => EndPosition { aim: acc.aim - v, ..acc },
            Instruction::Down(v) => EndPosition { aim: acc.aim + v, ..acc },
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_instructions_for_example() {
        let example = vec![
            "forward 5".into(),
            "down 5".into(),
            "forward 8".into(),
            "up 3".into(),
            "down 8".into(),
            "forward 2".into(),
        ];

        let expected = vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ];

        assert_eq!(read_instructions(&example[..]), expected);
    }

    #[test]
    fn test_find_end_position_for_example() {
        let example = vec![
            Instruction::Forward(5),
            Instruction::Down(5),
            Instruction::Forward(8),
            Instruction::Up(3),
            Instruction::Down(8),
            Instruction::Forward(2),
        ];

        assert_eq!(find_end_position(&example), EndPosition { horizontal: 15, vertical: 60, aim: 10 });
    }
}

