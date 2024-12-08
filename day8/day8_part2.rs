pub mod grid;
pub mod position;

use std::{
    collections::HashMap, fs::File, io::{prelude::*, BufReader}, path::Path
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
    let lines: Vec<Vec<char>> = lines_from_file("input.txt").into_iter().map(|l| l.chars().collect()).collect();

    let grid = Grid::new(lines);

    let mut antennae: HashMap<char, Vec<Position>> = HashMap::new();
    grid.for_each(|c, pos| {
        if c != '.' {
            if let Some(list) = antennae.get_mut(&c) {
                list.push(pos);
            } else {
                antennae.insert(c, vec!(pos));
            }
        }
    });

    println!("{:?}", antennae);

    let mut antinodes = grid.map(|_| false);
    for list in antennae.values() {
        for (i, &p1) in list.iter().enumerate() {
            for j in (i+1)..list.len() {
                let p2 = list[j];
                let diff = p2 - p1;
                mark_line(&mut antinodes, p1, diff);
            }
        }
    }

    // let mut res = String::new();
    // let mut old_y = 0;
    // antinodes.for_each(|c, pos| {
    //     if pos.y > old_y {
    //         res += "\n";
    //     }
    //     old_y = pos.y;
    //     res += if c { "#" } else { "." };
    // });
    // println!("{}",res);

    let mut sum = 0;
    antinodes.for_each(|an, _| if an { sum += 1 });

    println!("Part 2: {}", sum);
}

fn mark_line(antinodes: &mut Grid<bool>, base: Position, dir: Position) {
    let mut current = base;

    loop {
        let mut cc = antinodes.get_mut_pos(current);
        match cc {
            Some(true) => (),
            Some(false) => *cc.unwrap() = true,
            None => break,
        }
        current = current + dir;
    }

    current = base - dir;
    loop {
        let mut cc = antinodes.get_mut_pos(current);
        match cc {
            Some(true) => (),
            Some(false) => *cc.unwrap() = true,
            None => break,
        }
        current = current - dir;
    }
}