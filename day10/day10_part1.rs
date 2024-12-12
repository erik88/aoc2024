pub mod grid;
pub mod position;

use std::{
    collections::{HashSet}, fs::File, io::{prelude::*, BufReader}, path::Path
};

use grid::Grid;
use position::Position;

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

fn main() {
    let lines = lines_from_file("input.txt");
    let grid: Grid<u32> = Grid::new(lines.into_iter().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect());

    let mut sum = 0;
    grid.for_each_mut(|_, head_pos| {
        let mut ends: HashSet<Position> = HashSet::new();
        sum += find_trails(&grid, 0, head_pos, &mut ends);
    });

    println!("Part 1: {}", sum);
}

fn find_trails(grid: &Grid<u32>, head_val: u32, pos: Position, ends: &mut HashSet<Position>) -> i64 {
    if let Some(val) = grid.get_pos(pos) {
        if head_val == val {
            if val == 9 {
                if ends.insert(pos) {
                    return 1;
                } else {
                    return 0;
                }
            }
            return 
                find_trails(grid, head_val+1, pos + Position{x: 1, y: 0}, ends) +
                find_trails(grid, head_val+1, pos + Position{x: -1, y: 0}, ends) +
                find_trails(grid, head_val+1, pos + Position{x: 0, y: 1}, ends) +
                find_trails(grid, head_val+1, pos + Position{x: 0, y: -1}, ends);
        }
    }
    return 0;
}