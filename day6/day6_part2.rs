mod grid;
mod position;

use std::{
    fs::File, io::{prelude::*, BufReader}, path::Path
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

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up, Down, Left, Right
}

fn main() {
    let lines = lines_from_file("input.txt");
    let grid = Grid::new(
        lines.into_iter().map(|line| line.chars().collect()).collect()
    );

    let startpos = grid.find_first('^').unwrap();
    let covered_from_start: Grid<Option<Direction>> = grid.map(|_| None);

    let mut obstacle_steps = 0;
    let mut loop_count = 0;

    // let mut i = 0;
    // while i < 5000 {
    loop {
        let is_loop = is_looping(grid.clone(), covered_from_start.clone(), startpos, &mut obstacle_steps);

        match is_loop {
            Some(true) => loop_count += 1,
            Some(false) => (),
            None => break
        }
    //    println!("{} {} {}", i, obstacle_steps, loop_count);
    //    i += 1;
    }

    println!("Part 2: {}", loop_count);
}

fn is_looping(mut grid: Grid<char>, mut covered: Grid<Option<Direction>>, startpos: Position, obstacle_steps: &mut i32) -> Option<bool> {
    
    let mut x = startpos.x;
    let mut y = startpos.y;

    let mut dir = Direction::Up;
    let mut obstacle_placed = false;
    let mut steps = 0;

    loop {
        let (next_x, next_y): (i32, i32) = get_next(dir, x, y);
        let next_char = grid.get(next_x, next_y);

        if let Some(c) = next_char {
            if c == '#' {
                dir = turn_right(dir);
            } else {
                // Forward is clear
                if !obstacle_placed && steps == *obstacle_steps {
                    match covered.get(next_x, next_y) {
                        None => return None,
                        Some(None) => {
                            let c = grid.get_mut(next_x, next_y).unwrap();
                            *c = '#';
                            *obstacle_steps += 1;
                            obstacle_placed = true;
                            continue;
                        },
                        Some(Some(_)) => *obstacle_steps += 1
                    }
                }


                // Have I been here before?
                let cc = covered.get_mut(x, y).unwrap();
                match cc {
                    Some(previous_dir) => {
                        if *previous_dir == dir {
                            return Some(true)
                        }
                        // Otherwise, keep the first direction encountered
                        // This will detect "line loops"
                    },
                    None => *cc = Some(dir),
                }

                // Take a step
                x = next_x;
                y = next_y;
                steps += 1;

            }
        } else {
            if steps <= *obstacle_steps {
                return None
            } else {
                return Some(false)
            }
        }
    }
}

fn get_next(dir: Direction, x: i32, y: i32) -> (i32,i32) {
    match dir {
        Direction::Down => (x, y + 1),
        Direction::Up => (x, y - 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y)
    }
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::Down => Direction::Left,
        Direction::Up => Direction::Right,
        Direction::Left => Direction::Up,
        Direction::Right => Direction::Down
    }
}