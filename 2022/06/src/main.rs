use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| {
            l.expect("line")
        })
        .collect();
    let signal: &str = lines.first().expect("input on first line");

    let init_idx = find_init(signal);

    if let Some(idx) = init_idx {
        println!("last idx in init seq: {:?}", idx);
    } else {
        println!("didn't find init seq");
    }

    let msg_start_idx = find_msg_start(signal);

    if let Some(idx) = msg_start_idx {
        println!("last idx in start msg seq: {:?}", idx);
    } else {
        println!("didn't find init seq");
    }

    Ok(())
}

fn find_init(signal: &str) -> Option<usize> {
    find_uniq_sec(signal, 4)
}

fn find_msg_start(signal: &str) -> Option<usize> {
    find_uniq_sec(signal, 14)
}

fn find_uniq_sec(signal: &str, len: usize) -> Option<usize> {
    let chars_with_idx = signal.chars().enumerate()
        .map(|(i, c)| (i + 1, c))
        .collect::<Vec<(usize, char)>>();

    for window in chars_with_idx[..].windows(len) {
        let chars = window.iter().map(|(_, c)| c);
        if std::collections::BTreeSet::from_iter(chars).len() == len {
            let (last_idx, _) = window.last().unwrap();
            return Some(*last_idx)
        }

    }

    None
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_sample() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(find_init(input), Some(7));
        assert_eq!(find_msg_start(input), Some(19));
    }

    #[test]
    fn test_sample2() {
        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(find_init(input), Some(5));
        assert_eq!(find_msg_start(input), Some(23));
    }

    #[test]
    fn test_sample3() {
        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(find_init(input), Some(6));
        assert_eq!(find_msg_start(input), Some(23));
    }

    #[test]
    fn test_sample4() {
        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(find_init(input), Some(10));
        assert_eq!(find_msg_start(input), Some(29));
    }

    #[test]
    fn test_sample5() {
        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(find_init(input), Some(11));
        assert_eq!(find_msg_start(input), Some(26));
    }
}
