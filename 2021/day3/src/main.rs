use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = Box::new(BufReader::new(stdin));
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let bin_number_len = lines[0].len();

    let mut gamma_binary = String::with_capacity(bin_number_len);
    for col in 0..bin_number_len {
                select_column(&lines, col);
        gamma_binary.push(
            most_common_bit(
                &select_column(&lines, col)
            )
        );
    }

    let epsilon_binary = flip_bits(&gamma_binary);

    let gamma = i32::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_binary, 2).unwrap();

    println!("gamma: {}, epsilon: {}, power consumption: {}", gamma, epsilon, gamma * epsilon);

    Ok(())
}

fn most_common_bit(bits: &Vec<char>) -> char {
    let num_ones = bits.iter().filter(|v| **v == '1').count();
    let num_zeroes = bits.len() - num_ones;

    if num_ones > num_zeroes {
        '1'
    } else {
        '0'
    }
}

fn select_column(binary_nums: &Vec<String>, column: usize) -> Vec<char> {
    binary_nums.iter().map(|num| {
        let chars: Vec<char> = num.chars().collect();
        chars[column]
    })
    .collect()
}

fn flip_bits(binary_num: &str) -> String {
    let mut flipped = String::with_capacity(binary_num.len());
    for c in binary_num.chars() {
        if c == '1' {
            flipped.push('0');
        } else {
            flipped.push('1');
        }
    }

    flipped
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_most_common_bit_ones_more_common() {
        let data = vec!['1', '1', '1', '0'];
        assert_eq!(most_common_bit(&data), '1');
    }

    #[test]
    fn test_most_common_bit_zeroes_more_common() {
        let data = vec!['1', '0', '0', '0'];
        assert_eq!(most_common_bit(&data), '0');
    }

    #[test]
    fn test_select_column() {
        let data = vec![
            "0001".into(),
            "0110".into(),
            "1110".into(),
            "1111".into(),
            "0000".into(),
        ];

        assert_eq!(select_column(&data, 1), vec!['0', '1', '1', '1', '0']);
        assert_eq!(select_column(&data, 3), vec!['1', '0', '0', '1', '0']);
    }

    #[test]
    fn test_flip_bits() {
        assert_eq!(flip_bits("0000"), "1111");
        assert_eq!(flip_bits("1111"), "0000");
        assert_eq!(flip_bits("1100"), "0011");
    }
}

