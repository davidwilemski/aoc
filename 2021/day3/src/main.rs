use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = Box::new(BufReader::new(stdin));
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let bin_number_len = lines[0].len();

    let mut gamma_binary = String::with_capacity(bin_number_len);
    for col in 0..bin_number_len {
        gamma_binary.push(
            most_common_bit(
                &select_column(&lines, col),
                '1'
            )
        );
    }

    let epsilon_binary = flip_bits(&gamma_binary);

    let gamma = i32::from_str_radix(&gamma_binary, 2).unwrap();
    let epsilon = i32::from_str_radix(&epsilon_binary, 2).unwrap();

    println!("gamma: {}, epsilon: {}, power consumption: {}", gamma, epsilon, gamma * epsilon);


    let o2_gen_rating = find_o2_gen_rating(lines.clone());
    let co2_scrubber_rating = find_co2_scrubber_rating(lines.clone());
    let co2_scrubber_rating_int = i32::from_str_radix(&co2_scrubber_rating, 2).unwrap();
    let o2_gen_rating_int = i32::from_str_radix(&o2_gen_rating, 2).unwrap();

    println!(
        "co2 scrubber rating: {}, oxygen generation rating: {}, life support rating: {}",
        co2_scrubber_rating_int,
        o2_gen_rating_int,
        co2_scrubber_rating_int * o2_gen_rating_int
    );

    Ok(())
}

fn find_o2_gen_rating(lines: Vec<String>) -> String {
    let bin_number_len = lines[0].len();
    let mut filtered_lines = lines.clone();
    let mut o2_gen_rating = String::new();
    while filtered_lines.len() > 1 {
        for col in 0..bin_number_len {
            let most_common_bit = most_common_bit(
                &select_column(&filtered_lines, col),
                '1'
            );
            filtered_lines = filtered_lines.drain(..).filter(|num| {
                let chars: Vec<char> = num.chars().collect();
                chars[col] == most_common_bit
            }).collect();

            if filtered_lines.len() == 1 {
                o2_gen_rating = filtered_lines[0].clone();
            }
        }
    }

    o2_gen_rating
}

fn find_co2_scrubber_rating(lines: Vec<String>) -> String {
    let bin_number_len = lines[0].len();
    let mut filtered_lines = lines.clone();
    let mut co2_scrubber_rating = String::new();
    while filtered_lines.len() > 1 {
        for col in 0..bin_number_len {
            let most_common_bit = most_common_bit(
                &select_column(&filtered_lines, col),
                '1'
            );
            let least_common_bit = if most_common_bit == '1' {
                '0'
            } else {
                '1'
            };
            filtered_lines = filtered_lines.drain(..).filter(|num| {
                let chars: Vec<char> = num.chars().collect();
                chars[col] == least_common_bit
            }).collect();

            if filtered_lines.len() == 1 {
                co2_scrubber_rating = filtered_lines[0].clone();
            }
        }
    }

    co2_scrubber_rating
}

fn most_common_bit(bits: &Vec<char>, tie_val: char) -> char {
    let num_ones = bits.iter().filter(|v| **v == '1').count();
    let num_zeroes = bits.len() - num_ones;

    if num_ones > num_zeroes {
        '1'
    } else if num_zeroes > num_ones {
        '0'
    } else {
        tie_val
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
        assert_eq!(most_common_bit(&data, '1'), '1');
    }

    #[test]
    fn test_most_common_bit_zeroes_more_common() {
        let data = vec!['1', '0', '0', '0'];
        assert_eq!(most_common_bit(&data, '1'), '0');
    }

    #[test]
    fn test_most_common_bit_equal_frequency() {
        let data = vec!['1', '0', '1', '0'];
        assert_eq!(most_common_bit(&data, '1'), '1');
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

    #[test]
    fn test_o2_gen_rating_for_example() {
        let data = vec![
            "00100".into(),
            "11110".into(),
            "10110".into(),
            "10111".into(),
            "10101".into(),
            "01111".into(),
            "00111".into(),
            "11100".into(),
            "10000".into(),
            "11001".into(),
            "00010".into(),
            "01010".into(),
        ];

        assert_eq!(find_o2_gen_rating(data), String::from("10111"));
    }
}

