use std::collections::{HashMap, HashSet, VecDeque};

use aoc2024::{file, l2d::{direction::Direction, grid::Grid, position::Position}};

// ---

#[derive(Copy, Clone)]
struct Probe {
    pos: Position,
    dir: Direction,
    points: i64
}

#[derive(Debug)]
struct RecordSet {
    up: Option<i64>,
    down: Option<i64>,
    left: Option<i64>,
    right: Option<i64>
}

impl RecordSet {
    fn new() -> RecordSet {
        RecordSet {
            up: None,
            down: None,
            left: None,
            right: None
        }
    }

    fn set(&mut self, dir: Direction, r: i64) {
        match dir {
            Direction::Up => { self.up = Some(r); },
            Direction::Down => { self.down = Some(r); },
            Direction::Left => { self.left = Some(r); },
            Direction::Right => { self.right = Some(r); },
        }
    }

    fn get(&self, dir: Direction) -> Option<i64> {
        match dir {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

fn main() {
    let chars = file::chars_from_file("input.txt");
    let maze = Grid::new(chars);

    let (start, end) = get_start_end_pos(&maze);
    let fastest = find_fastest_paths(&maze, start, end);
    let scenic_squares = find_scenic_squares(&fastest, start, end);

    // 510 is too low
    // 514 is too low
    println!("Part 1: {:?}", fastest.get(&end));
    println!("Part 2: {}", scenic_squares.len());
   
}

fn find_scenic_squares(fastest: &HashMap<Position, RecordSet>, start: Position, end: Position) -> HashSet<Position> {
    let mut horizon: Vec<(Position, Direction)> = Vec::new();
    let mut visited: HashSet<Position> = HashSet::new();
    
    horizon.push((end, Direction::Up));

    while let Some((pos, facing)) = horizon.pop() {
        visited.insert(pos);

        if pos == start {
            continue;
        }

        let rs = fastest.get(&pos).unwrap();
        let points = rs.get(facing).unwrap();

        Direction::all().into_iter()
            .filter(|&d| d != facing) // Do not go back to facing-direction
            .for_each(|d| {
                let prev_pos = pos + d.pos();
                let prev_facing = d.opposite();
                if let Some(prev_rs) = fastest.get(&prev_pos) {
                    if let Some(prev_looking_to_current_cost) = prev_rs.get(prev_facing) {
                        if facing == prev_facing {
                            if prev_looking_to_current_cost == points - 1 {
                                horizon.push((prev_pos, prev_facing));
                            }
                        } else {
                            if prev_looking_to_current_cost == points - 1 - 1000 {
                                horizon.push((prev_pos, prev_facing));
                            }
                        }
                    }
                }
            });
    }
    visited
}

fn find_fastest_paths(maze: &Grid<char>, start: Position, end: Position) -> HashMap<Position, RecordSet> {
    let mut fastest: HashMap<Position, RecordSet> = HashMap::new();
    let mut horizon: VecDeque<Probe> = VecDeque::new();

    horizon.push_back(Probe { pos: start, dir: Direction::Right, points: 0 });

    while let Some(current) = horizon.pop_front() {
        check_square(current, &maze, end, &mut fastest, &mut horizon);
    }

    fastest
}

fn check_square(probe: Probe, maze: &Grid<char>, end: Position, fastest: &mut HashMap<Position, RecordSet>, horizon: &mut VecDeque<Probe>) {
    Direction::for_each(|dir| {
        // Don't go backwards
        if probe.dir.opposite() == dir {
            return;
        }

        let next_pos = probe.pos + dir.pos();
        if probe.pos != end {
            // Ignore directions that point into a wall,
            // except for the end (End will be nice to have when traveling backwards).
            match maze.get_pos(next_pos) {
                Some('#') | None => return,
                _ => (),
            }
        }

        let points = probe.points + if dir != probe.dir { 1000 } else { 0 };
        if insert_into_fastest(probe.pos, dir, points, fastest) {
            if probe.pos != end {
                horizon.push_back(Probe { pos: next_pos, dir, points: points + 1 });
            }
        }
    });
}

fn insert_into_fastest(pos: Position, dir: Direction, points: i64, fastest: &mut HashMap<Position, RecordSet>) -> bool {
    match fastest.get_mut(&pos) {
        Some(record_set) => {
            if let Some(record) = record_set.get(dir) {
                if points < record {
                    record_set.set(dir, points);
                    return true;    
                } else {
                    return false;
                }
            } else {
                record_set.set(dir, points);
                return true;
            }
        },
        None => {
            let mut rs = RecordSet::new();
            rs.set(dir, points);
            fastest.insert(pos, rs);
            return true;
        },
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

