
use std::collections::{HashMap, VecDeque};

use aoc2024::{file, l2d::{direction::Direction, grid::Grid, position::Position}};

// ---

#[derive(Copy, Clone)]
struct Probe {
    pos: Position,
    dir: Direction,
    points: i64
}

fn main() {
    let chars = file::chars_from_file("input.txt");
    let maze = Grid::new(chars);

    let (start, end) = get_start_end_pos(&maze);

    let mut fastest: HashMap<Position, i64> = HashMap::new();
    let mut horizon: VecDeque<Probe> = VecDeque::new();

    horizon.push_back(Probe { pos: start, dir: Direction::Right, points: 0 });

    while let Some(current) = horizon.pop_front() {
        check_neighbours(current, &maze, &mut fastest, &mut horizon);
    }

    println!("Part 1: {}", fastest.get(&end).unwrap());
}

fn check_neighbours(probe: Probe, maze: &Grid<char>, fastest: &mut HashMap<Position, i64>, horizon: &mut VecDeque<Probe>) {
    if probe.dir != Direction::Right { check_square(probe, Direction::Left, &maze, fastest, horizon); }
    if probe.dir != Direction::Left { check_square(probe, Direction::Right, &maze, fastest, horizon); }
    if probe.dir != Direction::Up { check_square(probe, Direction::Down, &maze, fastest, horizon); }
    if probe.dir != Direction::Down { check_square(probe, Direction::Up, &maze, fastest, horizon); }
}


fn check_square(probe: Probe, dir: Direction, maze: &Grid<char>, fastest: &mut HashMap<Position, i64>, horizon: &mut VecDeque<Probe>) {
    let mut at_goal = false;
    let new_pos = probe.pos + dir.pos();
    match maze.get_pos(new_pos) {
        Some('#') => {
            return
        },
        Some('.') | Some('S') => {
            ()
        },
        Some('E') => {
            at_goal = true;
        }
        Some(c) => panic!("Unexpected character {}", c),
        None => return,
    }

    let turn_points = if dir == probe.dir { 0 } else { 1000 };
    let new_points = probe.points + 1 + turn_points;
    let mut insert = false;

    match fastest.get_mut(&new_pos) {
        Some(record) => if *record > new_points {
             *record = new_points;
             insert = true;
        },
        None => {
            fastest.insert(new_pos, new_points);
            insert = true;
        },
    }


    if !at_goal && insert {
        horizon.push_back(Probe { pos: new_pos, dir, points: new_points });
        check_neighbours(probe, maze, fastest, horizon);
    }
}

fn get_start_end_pos(maze: &Grid<char>) -> (Position, Position) {
    let mut start: Position = Position { x: -1,y: -1 };
    let mut end: Position = Position { x: -1,y: -1 };

    maze.for_each_mut(|c, pos| match c {
        'S' => start = pos,
        'E' => end = pos,
        _ => (),
    });

    (start, end)
}