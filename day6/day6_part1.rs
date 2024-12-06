use std::{
    fs::File, io::{prelude::*, BufReader}, path::Path
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

// ---

enum Direction {
    Up, Down, Left, Right
}

fn main() {
    let lines = lines_from_file("input.txt");
    let grid: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect()).collect();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dir = Direction::Up;

    let mut xstart: usize = 0;
    let mut ystart: usize = 0;

    for (yi, row) in grid.iter().enumerate() {
        for (xi, char) in row.iter().enumerate() {
            if *char == '^' {
                x = xi.try_into().unwrap();
                y = yi.try_into().unwrap();
                xstart = xi;
                ystart = yi;
                break;
            }
        }
    }

    let mut covered: Vec<Vec<bool>> = grid.iter().map(|row| vec![false; row.len()]).collect();
    covered.get_mut(ystart).unwrap()[xstart] = true;
    let mut sum_covered = 1;

    loop {
        let oldx = x;
        let oldy = y;
        get_next(&dir, &mut x, &mut y);

        let indexy: Option<usize> = y.try_into().ok();
        let indexx: Option<usize> = x.try_into().ok();

        if indexx == None || indexy == None {
            break;
        }

        let next_char = grid.get(indexy.unwrap()).map(|row| row.get(indexx.unwrap()).map(|c| *c)).flatten();
        if let Some(c) = next_char {
            if c == '#' {
                x = oldx;
                y = oldy;
                dir = turn_right(dir);
            } else {
                let covered_square: &mut bool = covered.get_mut(indexy.unwrap()).unwrap().get_mut(indexx.unwrap()).unwrap();
                if !*covered_square {
                    sum_covered += 1;
                    *covered_square = true;
                }
            }
        } else {
            break;
        }
    }

    println!("Part 1: {}", sum_covered);
}

fn get_next(dir: &Direction, x: &mut i32, y: &mut i32) {
    match dir {
        Direction::Down => *y += 1,
        Direction::Up => *y -= 1,
        Direction::Left => *x -= 1,
        Direction::Right => *x += 1
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