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

fn main() {
    let lines = lines_from_file("input.txt");
    let height = lines.len();
    let width = lines.get(0).unwrap().len();

    let grid: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect()).collect();
    let mut sum: i64 = 0;

    for x in 0..width {
        for y in 0..height {
            let xu: i32 = x.try_into().unwrap();
            let yu: i32 = y.try_into().unwrap();
            sum += find_words_starting_from(xu, yu, &grid);
        }
    }

    println!("Part 1: {}", sum);
}

fn get(x: i32, y: i32, grid: &Vec<Vec<char>>) -> Option<char> {
    let xu: usize = x.try_into().ok()?;
    let yu: usize = y.try_into().ok()?;

    if let Some(row) = grid.get(yu) {
        if let Some(c) = row.get(xu) {
            return Some(*c);
        }
    }    
    
    None
}

fn find_words_starting_from(x: i32, y: i32, grid: &Vec<Vec<char>>) -> i64 {
    if get(x, y, grid).unwrap() != 'X' {
        return 0;
    }

    let mut sum = 0;

    sum += find_mas(x, y, 0, 1, grid);
    sum += find_mas(x, y, 1, 0, grid);
    sum += find_mas(x, y, -1, 0, grid);
    sum += find_mas(x, y, 0, -1, grid);

    sum += find_mas(x, y, 1, 1, grid);
    sum += find_mas(x, y, -1, -1, grid);
    sum += find_mas(x, y, -1, 1, grid);
    sum += find_mas(x, y, 1, -1, grid);

    return sum;
}

fn find_mas(x: i32, y: i32, dx: i32, dy: i32, grid: &Vec<Vec<char>>) -> i64 {
    if let Some(c) = get(x+dx, y+dy, grid) {
        if let Some(c2) = get(x+2*dx, y+2*dy, grid) {
            if let Some(c3) = get(x+3*dx, y+3*dy, grid) {
                if c == 'M' && c2 == 'A' && c3 == 'S' {
                    return 1
                }
            }
        }
    }

    0
}