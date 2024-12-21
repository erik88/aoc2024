use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

use aoc2024::{file, l2d::{grid::Grid, position::Position}};

// ---

// const FILE_NAME: &str = "test.txt";
// const REQUIRED_SAVE: i64 = 1;
// const FLIP_KEYS: bool = true;
const FILE_NAME: &str = "input.txt";
const REQUIRED_SAVE: i64 = 100;
const FLIP_KEYS: bool = false;

fn main() {
    let chars = file::chars_from_file(FILE_NAME);
    let mut maze = Grid::new(chars);

    let mut start = Position{x:0, y:0};
    let mut end = Position{x:0, y:0};

    maze.for_each_mut(|c, pos| {
        if c == 'S' {
            start = pos;
        } else if c == 'E' {
            end = pos;
        }
    });
    *maze.get_mut_pos(start).unwrap() = '.';
    *maze.get_mut_pos(end).unwrap() = '.';

    let mut lengths: HashMap<u64, u64> = HashMap::new();
    let standard_length = do_astar(&maze, start, end, 9999999999).unwrap();
    println!("Standard length: {}", standard_length);

    let mut iterations = 0;
    for cheat_x in 1..maze.width-1 {
        for cheat_y in 1..maze.height-1 {
            iterations += 1;
            let cheat_pos = Position{x: cheat_x.into(), y: cheat_y.into()};
            {
                if !(maze.get_pos(cheat_pos.left()).unwrap() == '.' && maze.get_pos(cheat_pos.right()).unwrap() == '.' || maze.get_pos(cheat_pos.up()).unwrap() == '.' && maze.get_pos(cheat_pos.down()).unwrap() == '.') {
                    continue;
                }
                // Mutable borrow has to happen after. What can you do.
                let cheat_c = maze.get_mut_pos(cheat_pos).unwrap();
                if *cheat_c != '#' {
                    continue;
                }
                *cheat_c = '.';
            }
            
            if let Some(length) = do_astar(&maze, start, end, standard_length) {
                if standard_length as i64 - REQUIRED_SAVE >= length as i64 {
                    let key: u64 = if FLIP_KEYS { standard_length-length } else { standard_length }.try_into().unwrap();
                    if let Some(count) = lengths.get_mut(&key) {
                        *count += 1;
                    } else {
                        lengths.insert(key, 1);
                    }
                }
            }

            *maze.get_mut_pos(cheat_pos).unwrap() = '#';

            if iterations % 100 == 0 {
                println!("Iteration {} of {}", iterations, (maze.width-2)*(maze.height-2))
            }
        }
    };

    println!("{:?}", lengths);
    let sum: u64 = lengths.into_values().sum();
    println!("Cheats {}", sum);
}

fn do_astar(maze: &Grid<char>, start: Position, end: Position, cutoff: u64) -> Option<u64> {
    let mut horizon: BinaryHeap<State> = BinaryHeap::new();
    let mut costs: HashMap<Position, u64> = HashMap::new();

    horizon.push(State { est: heuristic(start,end), pos: start, recorded_cost: 0});
    costs.insert(start, 0);
    let mut solution = 99999999999_u64;

    while let Some(state) = horizon.pop() {
        let pos = state.pos;
        let cost = costs.get(&pos).unwrap();
        if pos == end {
            solution = *cost;
            break;
        }

        if *cost < state.recorded_cost {
            // This node was updated, ignore
            continue;
        }

        if *cost + heuristic(pos, end) > cutoff {
            return None;
        }

        let new_cost = cost + 1;
        try_add(pos.up(), new_cost, &mut horizon, &mut costs, &maze, end);
        try_add(pos.down(), new_cost, &mut horizon, &mut costs, &maze, end);
        try_add(pos.left(), new_cost, &mut horizon, &mut costs, &maze, end);
        try_add(pos.right(), new_cost, &mut horizon, &mut costs, &maze, end);
    }

    Some(solution)
}

fn try_add(pos: Position, new_cost: u64, horizon: &mut BinaryHeap<State>, costs: &mut HashMap<Position, u64>, maze: &Grid<char>, end: Position) {
    if maze.get_pos(pos).unwrap() == '#' {
        return;
    }

    let new_state = State { pos, est: new_cost + heuristic(pos, end), recorded_cost: new_cost };
    if let Some(cost) = costs.get_mut(&pos) {
        if new_cost < *cost {
            *cost = new_cost;
            horizon.push(new_state);
        }
    } else {
        costs.insert(pos, new_cost);
        horizon.push(new_state);
    }
}

fn heuristic(p1: Position, p2: Position) -> u64 {
    let diff = p2-p1;
    if let Ok(num) = ((diff.x + diff.y).abs()).try_into() {
        return num;
    } else {
        println!("{} {}", p1, p2);
        panic!();
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct State {
    est: u64,
    pos: Position,
    recorded_cost: u64
}

// ----------------------------------------------------
// Borrowed from Rust std-documentation

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.est.cmp(&self.est)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}