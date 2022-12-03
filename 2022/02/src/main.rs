use std::io::prelude::*;
use std::io::BufReader;
// A Rock
// B Paper
// C Scissors
//
// X Rock 1
// Y Paper 2
// Z Scissors 3
//
// Win 6
// Draw 3
// Loss 0

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let lines: Vec<String> = reader
        .lines()
        .map(|l| {
            l.expect("line")
        })
        .collect();

    let char_lines: Vec<Vec<char>> = lines.iter().map(|l| {
        l.as_str().chars().map(convert_chars_v1).collect()
    }).collect();
    let score = char_lines.iter()
        .fold(0, |score: i32, chars: &Vec<char>| {
            score + score_round(&chars[..])
        });

    println!("score: {:?}", score);

    let score_round_2 =
        lines.iter()
        .map(|l| {
            match l.chars().collect::<Vec<char>>()[..] {
                [opp @ 'A', ' ', 'X' ] => [opp, ' ', 'C'],
                [opp, ' ', 'X'] => [opp, ' ', ((opp as u8) - 1) as char ],
                [opp, ' ', 'Y' ] => [opp, ' ', opp],
                [opp @ 'C', ' ', 'Z' ] => [opp, ' ', 'A'],
                [opp, ' ', 'Z'] => [opp, ' ', ((opp as u8) + 1) as char ],
                [] | [_, ..] => panic!("unhandled"),
            }
        })
        .fold(0, |score: i32, chars: [char; 3]| {
            score + score_round(&chars[..])
        });

    println!("part 2 score: {:?}", score_round_2);


    Ok(())
}

fn score_round(chars: &[char]) -> i32 {
    match chars {
        ['C', ' ', you @ 'A'] => play_score(*you) + 6,
        ['A', ' ', you @ 'C'] => play_score(*you),
        [opp, ' ', you] if you < opp => play_score(*you),
        [opp, ' ', you] if you > opp => play_score(*you) + 6,
        [opp, ' ', you] if opp == you => play_score(*you) + 3,
        [] => panic!("not handled"),
        [_, ..] => panic!("not handled"),
    }
}

fn play_score(c: char) -> i32 {
    c as i32 - 'A' as i32 + 1
}

fn convert_chars_v1(c: char) -> char {
    match c {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        val => val,
    }
}

