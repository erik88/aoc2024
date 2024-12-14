use std::{collections::HashSet, fs};

use aoc2024::{file, l2d::position::Position};
use regex::Regex;

// ---

struct Robot {
    pos: Position,
    v: Position
}

// const WIDTH: i64 = 11;
// const HEIGHT: i64 = 7;
// const FILE: &str = "test.txt";

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;
const FILE: &str = "input.txt";

fn main() {
    let lines = file::lines_from_file(FILE);
    let mut robots: Vec<Robot> = lines.into_iter().map(|r| read_robot(r)).collect();

    for i in 1..10000 {
        for robot in &mut robots {
            move_robot(robot);
        }
        if let Some(s) = print_robots_non_overlapping(&robots) {
            fs::write(format!("day14/{}.txt", i), s).unwrap();
            println!("No collisions: {}", i);
        }
    }
}

fn print_robots_non_overlapping(robots: &[Robot]) -> Option<String> {
    let mut set: HashSet<Position> = HashSet::new();
    for r in robots {
        if !set.insert(r.pos) {
            return None
        }
        
    }
    let mut s = String::new();
    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            s += if set.contains(&Position{ x, y }) { "#" } else { "." }
        }
        s += "\n";
    }
    Some(s)
}

fn move_robot(robot: &mut Robot) {
    robot.pos.x = (robot.pos.x + robot.v.x).rem_euclid(WIDTH);
    robot.pos.y = (robot.pos.y + robot.v.y).rem_euclid(HEIGHT);
}

fn read_robot(line: String) -> Robot {
    let r = Regex::new(r"p=(\d+),(\d+) v=(\-?\d+),(\-?\d+)").unwrap();
    let caps = r.captures(line.as_str()).unwrap();
    Robot {
        pos: Position {
            x: caps[1].parse().unwrap(),
            y: caps[2].parse().unwrap()
        },
        v: Position {
            x: caps[3].parse().unwrap(),
            y: caps[4].parse().unwrap()
        }
    }
}