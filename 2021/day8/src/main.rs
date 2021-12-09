use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = Box::new(BufReader::new(stdin));
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let note_lines: Vec<NoteLine> = lines.iter().map(|l| parse_line(l)).collect();

    let number_of_1_4_7_or_8s: i32 = note_lines.iter().map(|n| n.output.iter().filter(|o| is_1_4_7_8(o)).count()).sum::<usize>() as i32;

    println!("number of 1s, 4s, 7s, and 8s: {}", number_of_1_4_7_or_8s);

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct NoteLine {
    signals: Vec<String>,
    output: Vec<String>,
}

// bcedagf ebadf gcdfe gfcead bcedgf dfeca ac dgca ace cafbge | ecfdbag gecfd feadb degacbf
// Should result in the 10 sequences on left side being 10 entries in signals and 4 entries on
// right side of | being in output.
fn parse_line(line: &str) -> NoteLine {
    let signals_and_output: Vec<&str> = line.split(" | ").collect();

    NoteLine {
        signals: signals_and_output[0].split(' ').map(|s| String::from(s)).collect::<Vec<String>>(),
        output: signals_and_output[1].split(' ').map(|s| String::from(s)).collect::<Vec<String>>(),
    }
}

fn is_1_4_7_8(output_val: &str) -> bool {
    match output_val.len() {
        2 => true, // two segments is a 1
        4 => true, // four segments mean a 4
        3 => true, // three segments mean a 7
        7 => true, // seven segments mean an 8
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe";

        assert_eq!(parse_line(line), NoteLine {
            signals: vec![
                "be".into(),
                "cfbegad".into(),
                "cbdgef".into(),
                "fgaecd".into(),
                "cgeb".into(),
                "fdcge".into(),
                "agebfd".into(),
                "fecdb".into(),
                "fabcd".into(),
                "edb".into(),
            ],
            output: vec![
                "fdgacbe".into(),
                "cefdb".into(),
                "cefbgd".into(),
                "gcbe".into(),
            ],
        });
    }
}
