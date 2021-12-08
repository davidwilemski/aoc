use std::collections::BTreeSet;
use std::io::BufReader;
use std::io::prelude::*;
use std::iter::FromIterator;


fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = Box::new(BufReader::new(stdin));
    let mut lines = reader.lines();

    let bingo_draws: Vec<i32> = lines.next().unwrap()?.split(',').map(|v| v.parse::<i32>().unwrap()).collect();
    // lines.next(); // skip newline
    println!("bingo draws: {:?}", bingo_draws);

    let bingo_lines: Vec<String> = lines.map(|l| l.unwrap()).collect();
    let bingo_boards = build_bingo_boards(&bingo_lines);

    for i in 1..(bingo_draws.len()) {
        let selected: BTreeSet<i32> = BTreeSet::from_iter(bingo_draws[0..i].iter().cloned());
        println!("selected: {:?}", bingo_draws[0..i].iter());
        let bingos = bingo_boards.iter().filter(|b| b.has_bingo(&selected)).collect::<Vec<&BingoBoard>>();
        if !bingos.is_empty() {
            println!("{} board(s) have a bingo", bingos.len());
            for board in bingos {
                println!("board: {:?}", board);
                println!("score: {}", board.score(&selected, bingo_draws[i - 1]));
            }
            // guessed 37882 - too low
            // Manual scoring:
            // 86 + 77 + 87 + 79 + 52 + 17 + 20 + 30 + 48 + 25 + 13 + 9 + 47 + 45 + 97 + 15 + 59 = 806
            // last selected = 73
            // answer: 806 * 73 = 58838
            return Ok(());
        }
    }

    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
struct BingoBoard {
    board: Vec<i32>, // board (single vec storing 5x5 board)
}

impl BingoBoard {
    fn has_bingo(&self, selected_nums: &BTreeSet<i32>) -> bool {
        Self::valid_bingo_sequences().iter().any(|s| {
            s.iter().all(|idx| selected_nums.contains(&self.board[*idx]))
        })
    }

    fn score(&self, selected_nums: &BTreeSet<i32>, last_selected: i32) -> i32 {
        let not_selected_nums = BTreeSet::from_iter(self.board.clone().into_iter()).difference(selected_nums).map(|v| *v).collect::<Vec<i32>>();
        println!("not selected nums: {:?}", not_selected_nums);
        let not_selected_sum: i32 = BTreeSet::from_iter(self.board.iter().cloned()).difference(selected_nums).sum();
        println!("not selected sum: {}", not_selected_sum);
        println!("last selected num: {}", last_selected);

        not_selected_sum * last_selected
    }

    // Assume 5x5 bingo card for now
    fn valid_bingo_sequences() -> BTreeSet<BTreeSet<usize>>{
        let mut sequences: BTreeSet<BTreeSet<usize>> = (0..25).collect::<Vec<usize>>()
            .chunks(5)
            .map(|c| {
                let mut s = BTreeSet::new();
                s.extend(c.iter());
                s
            }).collect();

        // Just hardcoding instead of generating programmatically
        sequences.extend([
            BTreeSet::from([0, 5, 10 , 15, 20]),
            BTreeSet::from([1, 6, 11 , 16, 21]),
            BTreeSet::from([2, 7, 12 , 17, 22]),
            BTreeSet::from([3, 8, 13 , 18, 23]),
            BTreeSet::from([4, 9, 14 , 19, 24]),
        ]);

        sequences
    }
}

fn build_bingo_boards(lines: &Vec<String>) -> Vec<BingoBoard> {
    let mut bingo_boards = Vec::new();
    let mut lines_iter = lines.chunks(6);
    while let Some(next_chunk) = lines_iter.next() {
        let mut board_vec: Vec<i32> = Vec::new();
        let next_five = next_chunk[1..=5].iter();
        for board_line in next_five {
            board_vec.extend(board_line.trim().split(' ').filter(|s| *s != "").map(|s| {
                println!("{:?}", s);
                s.parse::<i32>().unwrap()
            }).collect::<Vec<i32>>());
        }
        println!("board_vec: {:?}", board_vec);
        bingo_boards.push(BingoBoard { board: board_vec });
    }

    bingo_boards
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_bingo_boards_with_example() {
        let example: Vec<String> = vec![
            "\n".into(),
            "22 13 17 11  0\n".into(),
            "8  2 23  4 24\n".into(),
            "21  9 14 16  7\n".into(),
            " 6 10  3 18  5\n".into(),
            " 1 12 20 15 19\n".into(),
            "\n".into(),
            " 3 15  0  2 22\n".into(),
            " 9 18 13 17  5\n".into(),
            "19  8  7 25 23\n".into(),
            "20 11 10 24  4\n".into(),
            "14 21 16 12  6\n".into(),
            "\n".into(),
            "14 21 17 24  4\n".into(),
            "10 16 15  9 19\n".into(),
            "18  8 23 26 20\n".into(),
            "22 11 13  6  5\n".into(),
            " 2  0 12  3  7\n".into(),
        ];

        assert_eq!(build_bingo_boards(&example), vec![
            BingoBoard { board: vec![22, 13, 17, 11, 0, 8, 2, 23, 4, 24, 21, 9, 14, 16, 7, 6, 10, 3, 18, 5, 1, 12, 20, 15, 19] },
            BingoBoard { board: vec![3, 15, 0, 2, 22, 9, 18, 13, 17, 5, 19, 8, 7, 25, 23, 20, 11, 10, 24, 4, 14, 21, 16, 12, 6] },
            BingoBoard { board: vec![14, 21, 17, 24, 4, 10, 16, 15, 9, 19, 18, 8, 23, 26, 20, 22, 11, 13, 6, 5, 2, 0, 12, 3, 7] },
        ]);

    }

    #[test]
    fn test_valid_bingo_sequences() {
        assert_eq!(
            BingoBoard::valid_bingo_sequences(),
            BTreeSet::from([
                // Horizontal bingo
                BTreeSet::from([0, 1, 2, 3, 4]),
                BTreeSet::from([5, 6, 7, 8, 9]),
                BTreeSet::from([10, 11, 12, 13, 14]),
                BTreeSet::from([15, 16, 17, 18, 19]),
                BTreeSet::from([20, 21, 22, 23, 24]),

                // Vertical bingo
                BTreeSet::from([0, 5, 10 , 15, 20]),
                BTreeSet::from([1, 6, 11 , 16, 21]),
                BTreeSet::from([2, 7, 12 , 17, 22]),
                BTreeSet::from([3, 8, 13 , 18, 23]),
                BTreeSet::from([4, 9, 14 , 19, 24]),
            ])
        );
    }

    #[test]
    fn test_has_bingo_with_example() {
        let selected = BTreeSet::from([7,4,9,5,11,17,23,2,0,14,21,24]);
        let example: Vec<String> = vec![
            "\n".into(),
            "22 13 17 11  0\n".into(),
            "8  2 23  4 24\n".into(),
            "21  9 14 16  7\n".into(),
            " 6 10  3 18  5\n".into(),
            " 1 12 20 15 19\n".into(),
            "\n".into(),
            " 3 15  0  2 22\n".into(),
            " 9 18 13 17  5\n".into(),
            "19  8  7 25 23\n".into(),
            "20 11 10 24  4\n".into(),
            "14 21 16 12  6\n".into(),
            "\n".into(),
            "14 21 17 24  4\n".into(),
            "10 16 15  9 19\n".into(),
            "18  8 23 26 20\n".into(),
            "22 11 13  6  5\n".into(),
            " 2  0 12  3  7\n".into(),
        ];
        let boards = build_bingo_boards(&example);

        assert_eq!(boards[0].has_bingo(&selected), false);
        assert_eq!(boards[1].has_bingo(&selected), false);
        assert_eq!(boards[2].has_bingo(&selected), true);
    }
}
