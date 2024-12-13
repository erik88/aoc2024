use aoc2024::{file, l2d::position::Position};
use regex::Regex;

// ---

#[derive(Debug)]
struct Machine {
    a: Position,
    b: Position,
    win: Position
}

fn main() {
    let lines = file::lines_from_file("input.txt");

    let machines = read_machines(lines);
    let mut sum_tokens = 0;
    for machine in machines {
        sum_tokens += calc_min_tokens(machine).unwrap_or(0);
    }

    println!("Part 1: {}", sum_tokens);
}

fn calc_min_tokens(machine: Machine) -> Option<i32> {
    let mut min_tokens: Option<i32> = None;
    for a_times in 0..101 {
        for b_times in 0..101 {
            let dest = machine.a*a_times + machine.b*b_times;
            if dest == machine.win {
                let tokens = 3*a_times + b_times;
                match min_tokens {
                    None => min_tokens = Some(tokens),
                    Some(t) => if tokens < t { min_tokens = Some(tokens) }
                }
            }
        }
    }
    min_tokens
}

fn read_machines(lines: Vec<String>) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    let mut state = 0;
    let mut a: Position = Position { x: 0, y: 0 };
    let mut b: Position = Position { x: 0, y: 0 };
    let mut win: Position;
    for line in lines {
        if line.is_empty() {
            continue;
        }
        if state == 0 {
            let r = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
            let caps = r.captures(line.as_str()).unwrap();
            let x: i32 = caps[1].parse().unwrap();
            let y: i32 = caps[2].parse().unwrap();
            a = Position { x, y };
            state = 1;
        } else if state == 1 {
            let r = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
            let caps = r.captures(line.as_str()).unwrap();
            let x: i32 = caps[1].parse().unwrap();
            let y: i32 = caps[2].parse().unwrap();
            b = Position { x, y };
            state = 2;  
        } else if state == 2 {
            let r = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
            let caps = r.captures(line.as_str()).unwrap();
            let x: i32 = caps[1].parse().unwrap();
            let y: i32 = caps[2].parse().unwrap();
            win = Position { x, y };
            machines.push(Machine { a, b, win });
            state = 0;
        }
    }
    return machines;
}