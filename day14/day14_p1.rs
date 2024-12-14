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

    for robot in &mut robots {
        move_robot(robot);
    }

    let (q1,q2,q3,q4) = count_robots_in_quadrants(robots);

    // Too low 215538750
    println!("Quads: {} {} {} {}", q1, q2, q3, q4);
    println!("Quadsum: {}", q1+q2+q3+q4);
    println!("Part 1: {}", q1*q2*q3*q4);
}

fn count_robots_in_quadrants(robots: Vec<Robot>) -> (i64, i64, i64, i64) {
    let (mut q1, mut q2, mut q3, mut q4) = (0, 0, 0, 0);
    for robot in robots {
        if robot.pos.x < (WIDTH-1)/2 {
            if robot.pos.y < (HEIGHT-1)/2 {
                q1 += 1;
            } else if robot.pos.y >= (HEIGHT+1)/2 {
                q2 += 1;
            }
        } else if robot.pos.x >= (WIDTH+1)/2 {
            if robot.pos.y < (HEIGHT-1)/2 {
                q3 += 1;
            } else if robot.pos.y >= (HEIGHT+1)/2 {
                q4 += 1;
            }
        }
    }
    (q1,q2,q3,q4)
}

fn move_robot(robot: &mut Robot) {
    robot.pos.x = (robot.pos.x + robot.v.x * 100).rem_euclid(WIDTH);
    robot.pos.y = (robot.pos.y + robot.v.y * 100).rem_euclid(HEIGHT);
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