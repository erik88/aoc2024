use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}};

use aoc2024::{file, l2d::{grid::Grid, position::Position}};

// ---

const WIDTH: i64 = 71;
const HEIGHT: i64 = 71;
const FILE: &str = "input.txt";
const FALLEN_SIZE: usize = 1024;
// const WIDTH: i64 = 7;
// const HEIGHT: i64 = 7;
// const FILE: &str = "test.txt";
// const FALLEN_SIZE: usize = 12;
const END: Position = Position {x: WIDTH-1, y: HEIGHT - 1};

#[derive(Debug)]
struct Fallen {
    positions: Vec<Position>
}

impl Fallen {
    fn is_fallen(&self, p: Position) -> bool {
        for s in &self.positions {
            if p == *s {
                return true
            }
        }
        false
    }
}

fn main() {
    let lines = file::lines_from_file(FILE);
    let fallen_pos: Vec<Position> = lines.into_iter()
    .map(|line| {
        let (a,b) = line.split_once(',').unwrap();
        Position {
            x: a.parse().unwrap(),
            y: b.parse().unwrap()
        }
    }).collect();
    let fallen = Fallen {
        positions: fallen_pos[0..FALLEN_SIZE].to_vec()
    };

    // Guessing part2 will be large and sparse, not using Grid-class
    //    let mut grid = Grid::new(numbers);

    let mut grid = Grid::from('.', WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap());
    for f in &fallen.positions {
        grid.try_set_pos('#', *f);
    }
    grid.print();

    let start = Position{x:0, y:0};
    let end = Position{x:WIDTH-1, y:HEIGHT-1};

    let mut horizon: BinaryHeap<State> = BinaryHeap::new();
    let mut costs: HashMap<Position, u64> = HashMap::new();

    horizon.push(State { est: heuristic(start,end), pos: start, recorded_cost: 0});
    costs.insert(start, 0);
    let mut solution = 99999999999_u64;
    let mut iterations = 0;

    while let Some(state) = horizon.pop() {
        iterations += 1;
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

        let new_cost = cost + 1;
        try_add(pos.up(), new_cost, &mut horizon, &mut costs, &fallen);
        try_add(pos.down(), new_cost, &mut horizon, &mut costs, &fallen);
        try_add(pos.left(), new_cost, &mut horizon, &mut costs, &fallen);
        try_add(pos.right(), new_cost, &mut horizon, &mut costs, &fallen);
    }

    println!("Iterations: {}", iterations);
    println!("Answer: {}", solution);
}

fn try_add(pos: Position, new_cost: u64, horizon: &mut BinaryHeap<State>, costs: &mut HashMap<Position, u64>, fallen: &Fallen) {
    if out_of_bounds(pos) {
        return;
    }

    if fallen.is_fallen(pos) {
        return;
    }

    let new_state = State { pos, est: new_cost + heuristic(pos, END), recorded_cost: new_cost };
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

#[inline]
fn out_of_bounds(pos: Position) -> bool {
    return !(0 <= pos.x && pos.x < WIDTH && 0 <= pos.y && pos.y < HEIGHT);
}

#[inline]
fn heuristic(p1: Position, p2: Position) -> u64 {
    let diff = p2-p1;
    if let Ok(num) = (diff.x + diff.y).try_into() {
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