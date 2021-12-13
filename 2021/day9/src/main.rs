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
    let mut low_point_idxs: Vec<usize> = Vec::new();
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
            low_point_idxs.push(idx);
        }
    }

    let risks = low_point_vals.iter().map(|v| v + 1).collect::<Vec<u32>>();

    println!("sum of risks: {}", risks.iter().sum::<u32>());

    let mut basin_sizes: Vec<usize> = low_point_idxs.iter().map(|idx| {
        let mut visited: HashSet<usize> = HashSet::new();
        // visited.insert(*idx);
        basin_size(*idx, &grid, grid_width, grid_height, &mut visited)
    }).collect();

    basin_sizes.sort();
    basin_sizes.reverse();

    let three_largest = basin_sizes.get(0..3).unwrap();
    println!("Three largest basin_sizes: {:?}, product: {}", three_largest, three_largest.iter().product::<usize>());

    Ok(())
}

// Do a depth first search in each direction (left, right, up, and down) until reaching a 9 (no
// longer in the basin). Returns the size of the basin.
fn basin_size(lowpoint: usize, grid: &Vec<u32>, grid_width: usize, grid_height: usize, visited: &mut HashSet<usize>) -> usize {
    if grid[lowpoint] == 9 || visited.contains(&lowpoint) {
        visited.insert(lowpoint);
        return 0;
    }
    visited.insert(lowpoint);

    // eprintln!("visited: {:?}", visited);
    let above_size: usize = if !is_top_row(lowpoint, grid_width) && !visited.contains(&(lowpoint - grid_width)) {
        // visited.insert(lowpoint - grid_width);
        basin_size(lowpoint - grid_width, grid, grid_width, grid_height, visited)
    } else {
        0
    };

    let below_size: usize = if !is_bottom_row(lowpoint, grid_width, grid_height) && !visited.contains(&(lowpoint + grid_width)) {
        // visited.insert(lowpoint + grid_width);
        basin_size(lowpoint + grid_width, grid, grid_width, grid_height, visited)
    } else {
        0
    };

    let left_size: usize = if !is_left_side(lowpoint, grid_width, grid_height) && !visited.contains(&(lowpoint - 1)) {
        // visited.insert(lowpoint - 1);
        basin_size(lowpoint - 1, grid, grid_width, grid_height, visited)
    } else {
        0
    };

    let right_size: usize = if !is_right_side(lowpoint, grid_width, grid_height) && !visited.contains(&(lowpoint + 1)) {
        // visited.insert(lowpoint + 1);
        basin_size(lowpoint + 1, grid, grid_width, grid_height, visited)
    } else {
        0
    };

    // eprintln!("idx: {}, above_size: {}, below_size: {}, left_size: {}, right_size: {}", lowpoint, above_size, below_size, left_size, right_size);

    1 + (above_size + below_size + left_size + right_size)
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

    #[test]
    fn test_basin_size() {
        let grid: Vec<u32> = vec![
            2,1,9,9,9,4,3,2,1,0,
            3,9,8,7,8,9,4,9,2,1,
            9,8,5,6,7,8,9,8,9,2,
            8,7,6,7,8,9,6,7,8,9,
            9,8,9,9,9,6,5,6,7,8,
        ];

        assert_eq!(basin_size(1, &grid, 10, 5, &mut HashSet::new()), 3);
    }
}
