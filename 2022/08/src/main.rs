use std::io::prelude::*;
use std::io::BufReader;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin);

    let trees: Vec<Vec<u32>> = reader
        .lines()
        .map(|l| {
            l.expect("line").chars().map(|c| c.to_digit(10).unwrap()).collect()
        })
        .collect();

    let rows = trees.len();
    let cols = trees.first().unwrap().len();

    let mut visible = 0;
    for i in 0..rows {
        for j in 0..cols {
            match (i, j) {
                (0, _) => visible += 1,
                (i, _) if i == rows - 1 => visible += 1,
                (_, 0) => visible += 1,
                (_, j) if j == cols - 1 => visible += 1,
                (i, j) => {
                    if is_visible(i, j, &trees) {
                        visible += 1;
                    } 
                },
            }
        }
    }

    println!("num visible trees: {}", visible);

    let mut best_coords: (usize, usize) = (999, 999);
    let mut best_scenic_score = 0;
    for i in 0..rows {
        for j in 0..cols {
            let score = view_score(i, j, &trees, false);
            if score > best_scenic_score {
                best_scenic_score = score;
                best_coords = (i, j)
            }
        }
    }

    // view_score(best_coords.0, best_coords.1, &trees, true);
    // view_score(3, 2, &trees, true);
    println!("best scenic score: {}", best_scenic_score);

    Ok(())
}

fn is_visible(row: usize, col: usize, trees: &Vec<Vec<u32>>) -> bool {
    let height = trees[row][col];
    // look for horizontal and vertical sight lines from (row, col)
    //look above (row, col)
    let mut above_heights = trees[0..row].iter().map(|r| r[col]);
    if above_heights.all(|h| h < height) {
        return true;
    }

    //look below (row, col)
    let mut below_heights = trees[row+1..].iter().map(|r| r[col]);
    if below_heights.all(|h| h < height) {
        return true;
    }

    //look left of (row, col)
    if trees[row][0..col].iter().all(|h| *h < height) {
        return true;
    }

    //look right of (row, col)
    if trees[row][col+1..].iter().all(|h| *h < height) {
        return true;
    }

    false
}

fn view_score(row: usize, col: usize, trees: &Vec<Vec<u32>>, log_scores: bool) -> u32 {
    let height = trees[row][col];
    // view distance left
    let mut above_heights = trees[0..row].iter().map(|r| r[col]).collect::<Vec<u32>>();
    // must reverse before iterating through so we start at the values nearest the tree
    above_heights.reverse();
    let above_distance =
        if above_heights[..].iter().all(|h| *h < height) {
            above_heights.len() as u32
        } else {
            above_heights[..].iter().take_while(|h| **h < height).count() as u32 + 1
        };

    let below_heights = trees[row+1..].iter().map(|r| r[col]).collect::<Vec<u32>>();
    let below_distance = 
        if below_heights[..].iter().all(|h| *h < height) {
            below_heights.len() as u32
        } else {
            below_heights[..].iter().take_while(|h| **h < height).count() as u32 + 1
        };

    let mut left_heights = trees[row][0..col].iter().collect::<Vec<&u32>>();
    // must reverse before iterating through so we start at the values nearest the tree
    left_heights.reverse();
    let left_distance =
        if left_heights[..].iter().all(|h| **h < height) {
            left_heights.len() as u32
        } else {

            left_heights[..].iter().take_while(|h| ***h < height).count() as u32 + 1
        };

    let right_heights = trees[row][col+1..].iter().collect::<Vec<&u32>>();
    let right_distance =
        if right_heights[..].iter().all(|h| **h < height) {
            right_heights.len() as u32
        } else {
            right_heights[..].iter().take_while(|h| ***h < height).count() as u32 + 1
        };
    if log_scores {
        println!("coord: {}, {}", row, col);
    println!("above heights: {:?}", above_heights);
        println!("above score: {}", above_distance);
        println!("below score: {}", below_distance);
        println!("left score: {}", left_distance);
        println!("right score: {}", right_distance);
    }

    above_distance * below_distance * left_distance * right_distance
}
