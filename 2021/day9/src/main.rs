use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::io::BufReader;
use std::iter::FromIterator;

fn main() -> Result<(), std::io::Error> {
    let stdin = std::io::stdin();
    let reader = Box::new(BufReader::new(stdin));
    let lines = reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let grid_width = lines[0].len();
    let grid_height = lines.len();
    let grid: Vec<u32> = lines.iter()
        .flat_map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>())
        .collect();

    let mut low_point_vals: Vec<u32> = Vec::new();
    for (idx, val) in grid.iter().enumerate() {
        let lower_than_vertical_adjacent = if is_top_row(idx, grid_width) {
            *val < grid[idx + grid_width]
        } else if is_bottom_row(idx, grid_width, grid_height) {
            *val < grid[idx - grid_width]
        } else { // check both above and below
            *val < grid[idx - grid_width] && *val < grid[idx + grid_width]
        };

        let lower_than_horizontal_adjacent = if is_right_side(idx, grid_width, grid_height) {
            *val < grid[idx - 1]
        } else if is_left_side(idx, grid_width, grid_height) {
            *val < grid[idx + 1]
        } else { // check both left and right spots
            *val < grid[idx - 1] && *val < grid[idx + 1]
        };

        if lower_than_horizontal_adjacent && lower_than_vertical_adjacent {
            low_point_vals.push(*val);
        }
    }

    let risks = low_point_vals.iter().map(|v| v + 1).collect::<Vec<u32>>();

    println!("sum of risks: {}", risks.iter().sum::<u32>());

    Ok(())
}

fn is_top_row(idx: usize, grid_width: usize) -> bool {
    (idx as i32 - grid_width as i32) < 0
}

fn is_bottom_row(idx: usize, grid_width: usize, grid_height: usize) -> bool {
    (idx + grid_width) >= grid_height * grid_width
}

fn is_right_side(idx: usize, grid_width: usize, grid_height: usize) -> bool {
    idx % grid_width == grid_width - 1
}

fn is_left_side(idx: usize, grid_width: usize, grid_height: usize) -> bool {
    idx % grid_width == 0
}

/*
 *  010
 *  111
 *  000
 *
 *
 */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_top_row() {
        let grid = vec![0, 1, 0, 1, 1, 1, 0, 0, 0];
        let grid_width = 3;
        let grid_height = 3;

        assert_eq!(is_top_row(0, grid_width), true);
        assert_eq!(is_top_row(2, grid_width), true);
        assert_eq!(is_top_row(3, grid_width), false);
        assert_eq!(is_top_row(6, grid_width), false);
        assert_eq!(is_top_row(8, grid_width), false);
    }

    #[test]
    fn test_is_bottom_row() {
        let grid = vec![0, 1, 0, 1, 1, 1, 0, 0, 0];
        let grid_width = 3;
        let grid_height = 3;

        assert_eq!(is_bottom_row(0, grid_width, grid_height), false);
        assert_eq!(is_bottom_row(2, grid_width, grid_height), false);
        assert_eq!(is_bottom_row(3, grid_width, grid_height), false);
        assert_eq!(is_bottom_row(6, grid_width, grid_height), true);
        assert_eq!(is_bottom_row(8, grid_width, grid_height), true);
    }

    #[test]
    fn test_is_right_side() {
        let grid = vec![0, 1, 0, 1, 1, 1, 0, 0, 0];
        let grid_width = 3;
        let grid_height = 3;

        assert_eq!(is_right_side(0, grid_width, grid_height), false);
        assert_eq!(is_right_side(2, grid_width, grid_height), true);
        assert_eq!(is_right_side(3, grid_width, grid_height), false);
        assert_eq!(is_right_side(4, grid_width, grid_height), false);
        assert_eq!(is_right_side(5, grid_width, grid_height), true);
        assert_eq!(is_right_side(6, grid_width, grid_height), false);
        assert_eq!(is_right_side(8, grid_width, grid_height), true);
    }

    #[test]
    fn test_is_left_side() {
        let grid = vec![0, 1, 0, 1, 1, 1, 0, 0, 0];
        let grid_width = 3;
        let grid_height = 3;

        assert_eq!(is_left_side(0, grid_width, grid_height), true);
        assert_eq!(is_left_side(2, grid_width, grid_height), false);
        assert_eq!(is_left_side(3, grid_width, grid_height), true);
        assert_eq!(is_left_side(4, grid_width, grid_height), false);
        assert_eq!(is_left_side(5, grid_width, grid_height), false);
        assert_eq!(is_left_side(6, grid_width, grid_height), true);
        assert_eq!(is_left_side(8, grid_width, grid_height), false);
    }
}
