use aoc2024::{file, l2d::position::Position};
use nalgebra::{Matrix2, Matrix2x1};
use regex::Regex;

// ---

#[derive(Debug)]
struct Machine {
    no: u32,
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

    // too big 161394478351831
    //         157102004711699
    // ok       95688837203288
    println!("Part 2: {}", sum_tokens);
}

fn calc_min_tokens(machine: Machine) -> Option<i64> {
    let m = Matrix2::new(machine.a.x as f64, machine.b.x as f64,
        machine.a.y as f64, machine.b.y as f64);  

    let inv = m.try_inverse()?;
    let win_mat = Matrix2x1::new(machine.win.x as f64,
        machine.win.y as f64);

    let res = inv*win_mat;
    let a_times = res[0].round() as i64;
    let b_times = res[1].round() as i64;

    if a_times < 0 || b_times < 0 {
        return None;
    }

    if a_times*machine.a.x + b_times*machine.b.x != machine.win.x ||
        a_times*machine.a.y + b_times*machine.b.y != machine.win.y {
            return None;
        }

    Some(3*a_times + b_times)
}

fn read_machines(lines: Vec<String>) -> Vec<Machine> {
    let mut machines: Vec<Machine> = Vec::new();
    let mut state = 0;
    let mut a: Position = Position { x: 0, y: 0 };
    let mut b: Position = Position { x: 0, y: 0 };
    let mut win: Position;
    for (id, line) in lines.into_iter().enumerate() {
        if line.is_empty() {
            continue;
        }
        if state == 0 {
            let r = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
            let caps = r.captures(line.as_str()).unwrap();
            let x: i64 = caps[1].parse().unwrap();
            let y: i64 = caps[2].parse().unwrap();
            a = Position { x, y };
            state = 1;
        } else if state == 1 {
            let r = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
            let caps = r.captures(line.as_str()).unwrap();
            let x: i64 = caps[1].parse().unwrap();
            let y: i64 = caps[2].parse().unwrap();
            b = Position { x, y };
            state = 2;  
        } else if state == 2 {
            let r = Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();
            let caps = r.captures(line.as_str()).unwrap();
            let x: i64 = caps[1].parse().unwrap();
            let y: i64 = caps[2].parse().unwrap();
            win = Position { x: x + 10000000000000, y: y + 10000000000000};
            machines.push(Machine { a, b, win, no: id.try_into().unwrap() });
            state = 0;
        }
    }
    return machines;
}