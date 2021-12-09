use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = Box::new(BufReader::new(stdin));
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let note_lines: Vec<NoteLine> = lines.iter().map(|l| parse_line(l)).collect();

    let number_of_1_4_7_or_8s: i32 = note_lines.iter().map(|n| n.output.iter().filter(|o| is_1_4_7_8(o)).count()).sum::<usize>() as i32;

    println!("number of 1s, 4s, 7s, and 8s: {}", number_of_1_4_7_or_8s);

    let sum_of_displays: i32 = note_lines.iter().map(|n| {
        let signals = reverse_signal_map(&determine_signals(&n.signals));
        determine_display_value(&signals, &n.output)
    }).sum();

    println!("sum of all display values: {}", sum_of_displays);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct NoteLine {
    signals: Vec<HashSet<char>>,
    output: Vec<HashSet<char>>,
}

// bcedagf ebadf gcdfe gfcead bcedgf dfeca ac dgca ace cafbge | ecfdbag gecfd feadb degacbf
// Should result in the 10 sequences on left side being 10 entries in signals and 4 entries on
// right side of | being in output.
fn parse_line(line: &str) -> NoteLine {
    let signals_and_output: Vec<&str> = line.split(" | ").collect();

    NoteLine {
        signals: signals_and_output[0].split(' ').map(|s| HashSet::from_iter(s.chars())).collect::<Vec<HashSet<char>>>(),
        output: signals_and_output[1].split(' ').map(|s| HashSet::from_iter(s.chars())).collect::<Vec<HashSet<char>>>(),
    }
}

fn is_1_4_7_8(output_val: &HashSet<char>) -> bool {
    match output_val.len() {
        2 => true, // two segments is a 1
        4 => true, // four segments mean a 4
        3 => true, // three segments mean a 7
        7 => true, // seven segments mean an 8
        _ => false,
    }
}

fn determine_value(signals: &Vec<HashSet<char>>, output: &HashSet<char>) -> i32 {
    signals.iter().enumerate().find_map(|(idx, val)| {
        if val == output {
            Some(idx)
        } else {
            None
        }
    }).unwrap() as i32
}

fn determine_display_value(signals: &Vec<HashSet<char>>, output: &Vec<HashSet<char>>) -> i32 {
    // requires signals to be a properly ordered signal map in vector form (see reverse_signal_map)
    (1000 * determine_value(signals, &output[0])) +
        (100 * determine_value(signals, &output[1])) +
         (10 * determine_value(signals, &output[2])) +
          determine_value(signals, &output[3])
}

// align signal set to the corresponding index
// e.g. the hashset signal for zero is at index 0, etc.
fn reverse_signal_map(signal_map: &HashMap<i32, HashSet<char>>) -> Vec<HashSet<char>> {
    let mut reversed = Vec::new();

    for (key, value) in signal_map {
        reversed.push((value.clone(), key));
    }

    reversed.sort_by_key(|(_, v)| v.clone());
    reversed.iter().map(|(k, _)| k.clone()).collect()
}

// find signal combos for all numbers 0-9
fn determine_signals(signals: &Vec<HashSet<char>>) -> HashMap<i32, HashSet<char>> {
    let one: HashSet<char> = signals.iter().cloned().filter(|cs| cs.len() == 2).next().unwrap();
    let four: HashSet<char> = signals.iter().cloned().filter(|cs| cs.len() == 4).next().unwrap();
    let seven: HashSet<char> = signals.iter().cloned().filter(|cs| cs.len() == 3).next().unwrap();
    let eight: HashSet<char> = signals.iter().cloned().filter(|cs| cs.len() == 7).next().unwrap();

    // 6, 9, 0
    let contains_6_signals = signals.iter().cloned().filter(|cs| cs.len() == 6);

    let maybe_six = contains_6_signals.clone().filter(|cs| one.difference(cs).count() == 1).collect::<Vec<HashSet<char>>>();
    assert_eq!(maybe_six.len(), 1);
    let six = maybe_six[0].clone();

    let mut contains_0_and_9 = contains_6_signals.clone()
        .filter(|cs| cs != &six)
        .collect::<Vec<HashSet<char>>>();
    // 0 filtered by 4 will leave 3 segments while 9 filtered by 3 leaves 2 segments
    // 9 is first, 0 second
    contains_0_and_9.sort_by_key(|cs| cs.difference(&four).count());
    assert!(contains_0_and_9.len() == 2);
    let nine = contains_0_and_9[0].clone();
    let zero = contains_0_and_9[1].clone();

    // one filtered by five or two leaves 1 segment
    // one filtered by three leaves 0 segments
    let contains_5_signals = signals.iter().cloned().filter(|cs| cs.len() == 5);
    let maybe_three = contains_5_signals.clone()
        .filter(|cs| one.difference(cs).count() == 0)
        .collect::<Vec<HashSet<char>>>();
    assert!(maybe_three.len() == 1);
    let three = maybe_three[0].clone();

    let mut contains_5_and_2 = contains_5_signals
        .filter(|cs| cs != &three)
        .collect::<Vec<HashSet<char>>>();
    assert!(contains_5_and_2.len() == 2);

    // four filtered by five leaves 1 signal
    // four filtered by two leaves 2 signals
    // five will be first and two will be second
    contains_5_and_2.sort_by_key(|cs| four.difference(cs).count());
    let five = contains_5_and_2[0].clone();
    let two = contains_5_and_2[1].clone();

    HashMap::from([
        (0, zero),
        (1, one),
        (2, two),
        (3, three),
        (4, four),
        (5, five),
        (6, six),
        (7, seven),
        (8, eight),
        (9, nine),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";

        assert_eq!(parse_line(line), NoteLine {
            signals: vec![
                HashSet::from_iter("be".chars()),
                HashSet::from_iter("cfbegad".chars()),
                HashSet::from_iter("cbdgef".chars()),
                HashSet::from_iter("fgaecd".chars()),
                HashSet::from_iter("cgeb".chars()),
                HashSet::from_iter("fdcge".chars()),
                HashSet::from_iter("agebfd".chars()),
                HashSet::from_iter("fecdb".chars()),
                HashSet::from_iter("fabcd".chars()),
                HashSet::from_iter("edb".chars()),
            ],
            output: vec![
                HashSet::from_iter("fdgacbe".chars()),
                HashSet::from_iter("cefdb".chars()),
                HashSet::from_iter("cefbgd".chars()),
                HashSet::from_iter("gcbe".chars()),
            ],
        });
    }

    /*
    acedgfb: 8
    cdfbe: 5
    gcdfa: 2
    fbcad: 3
    dab: 7
    cefabd: 9
    cdfgeb: 6
    eafb: 4
    cagedb: 0
    ab: 1
     */
    #[test]
    fn it_can_determine_numbers_for_signals() {

        let signals: Vec<HashSet<char>> = vec![
            HashSet::from_iter("acedgfb".chars()),
            HashSet::from_iter("cdfbe".chars()),
            HashSet::from_iter("gcdfa".chars()),
            HashSet::from_iter("fbcad".chars()),
            HashSet::from_iter("dab".chars()),
            HashSet::from_iter("cefabd".chars()),
            HashSet::from_iter("cdfgeb".chars()),
            HashSet::from_iter("eafb".chars()),
            HashSet::from_iter("cagedb".chars()),
            HashSet::from_iter("ab".chars()),
        ];

        let nums_for_signals = determine_signals(&signals);

        assert_eq!(
            nums_for_signals,
            HashMap::from([
                (0, HashSet::from_iter("cagedb".chars())),
                (1, HashSet::from_iter("ab".chars())),
                (2, HashSet::from_iter("gcdfa".chars())),
                (3, HashSet::from_iter("fbcad".chars())),
                (4, HashSet::from_iter("eafb".chars())),
                (5, HashSet::from_iter("cdfbe".chars())),
                (6, HashSet::from_iter("cdfgeb".chars())),
                (7, HashSet::from_iter("dab".chars())),
                (8, HashSet::from_iter("acedgfb".chars())),
                (9, HashSet::from_iter("cefabd".chars())),
            ])
        );
    }

    #[test]
    fn it_can_reverse_signal_maps_for_signals() {
        let signal_map = HashMap::from([
            (0, HashSet::from_iter("cagedb".chars())),
            (1, HashSet::from_iter("ab".chars())),
            (2, HashSet::from_iter("gcdfa".chars())),
            (3, HashSet::from_iter("fbcad".chars())),
            (4, HashSet::from_iter("eafb".chars())),
            (5, HashSet::from_iter("cdfbe".chars())),
            (6, HashSet::from_iter("cdfgeb".chars())),
            (7, HashSet::from_iter("dab".chars())),
            (8, HashSet::from_iter("acedgfb".chars())),
            (9, HashSet::from_iter("cefabd".chars())),
        ]);

        assert_eq!(
            reverse_signal_map(&signal_map),
            vec![
                HashSet::from_iter("cagedb".chars()),
                HashSet::from_iter("ab".chars()),
                HashSet::from_iter("gcdfa".chars()),
                HashSet::from_iter("fbcad".chars()),
                HashSet::from_iter("eafb".chars()),
                HashSet::from_iter("cdfbe".chars()),
                HashSet::from_iter("cdfgeb".chars()),
                HashSet::from_iter("dab".chars()),
                HashSet::from_iter("acedgfb".chars()),
                HashSet::from_iter("cefabd".chars()),
            ]
        );
    }
}
