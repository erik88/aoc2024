use std::{collections::HashSet, hash::Hasher, hash::Hash};

use aoc2024::{file, l2d::{grid::Grid, position::Position}};

// ---

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}
impl Direction {
    fn invert(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Clone)]
struct Fence {
    p1: Position,
    p2: Position
}

impl Fence {
    fn up(&self) -> Fence {
        self.offset(Position { x: 0, y: -1})
    }
    fn down(&self) -> Fence {
        self.offset(Position { x: 0, y: 1})
    }
    fn left(&self) -> Fence {
        self.offset(Position { x: -1, y: 0})
    }
    fn right(&self) -> Fence {
        self.offset(Position { x: 1, y: 0})
    }

    fn offset(&self, pos: Position) -> Fence {
        Fence {
            p1: self.p1 + pos,
            p2: self.p2 + pos,
        }
    }

    fn offset_dir(&self, dir: Direction) -> Fence {
        match dir {
            Direction::Up => self.up(),
            Direction::Down => self.down(),
            Direction::Left => self.left(),
            Direction::Right => self.right(),
        }
    }

    fn cross_up(&self) -> (Fence, Fence) {
        (
            Fence {
                p1: self.p1,
                p2: self.p1.up()
            },
            Fence {
                p1: self.p2,
                p2: self.p2.up()
            }
        )
    }
    fn cross_down(&self) -> (Fence, Fence) {
        (
            Fence {
                p1: self.p1,
                p2: self.p1.down()
            },
            Fence {
                p1: self.p2,
                p2: self.p2.down()
            }
        )
    }
    fn cross_left(&self) -> (Fence, Fence) {
        (
            Fence {
                p1: self.p1,
                p2: self.p1.left()
            },
            Fence {
                p1: self.p2,
                p2: self.p2.left()
            }
        )
    }
    fn cross_right(&self) -> (Fence, Fence) {
        (
            Fence {
                p1: self.p1,
                p2: self.p1.right()
            },
            Fence {
                p1: self.p2,
                p2: self.p2.right()
            }
        )
    }

    fn cross(&self, dir: Direction) -> (Fence, Fence) {
        match dir {
            Direction::Down => self.cross_down(),
            Direction::Up => self.cross_up(),
            Direction::Left => self.cross_left(),
            Direction::Right => self.cross_right(),

        }
    }
}

impl Hash for Fence {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.p1.x < self.p2.x || self.p1.x == self.p2.x && self.p1.y < self.p2.y {
            state.write_i32(self.p1.x);
            state.write_i32(self.p1.y);
            state.write_i32(self.p2.x);
            state.write_i32(self.p2.y);
        } else {
            state.write_i32(self.p2.x);
            state.write_i32(self.p2.y);
            state.write_i32(self.p1.x);
            state.write_i32(self.p1.y);
        }
    }
}

impl PartialEq for Fence {
    fn eq(&self, other: &Self) -> bool {
        self.p1 == other.p1 && self.p2 == other.p2 || self.p1 == other.p2 && self.p2 == other.p1
    }
}

impl Eq for Fence {}

struct Region {
    pub id: u32,
    plots: Vec<Position>,
    pub perimeter: u64,
    pub sides: u64
}

impl Region {
    fn new(id: u32) -> Region {
        Region {
            id,
            plots: Vec::new(),
            perimeter: 0,
            sides: 0
        }
    }

    fn add(&mut self, p: Position) {
        self.plots.push(p);
    }

    fn area(&self) -> u64 {
        self.plots.len().try_into().unwrap()
    }
}

fn main() {
    let chars = file::chars_from_file("input.txt");
    let grid = Grid::new(chars);
    let mut visited = grid.map(|_| 0);
    let mut regions: Vec<Region> = Vec::new();
    let mut region_id = 1;

    grid.for_each_mut(|c,pos| {
        if visited.get_pos(pos).unwrap() > 0 {
            return;
        }

        let mut reg = Region::new(region_id);
        region_id += 1;
        let mut perimeter: HashSet<Fence> = HashSet::new();
        flood(c, pos, &mut reg, &grid, &mut visited, &mut perimeter);
        reg.perimeter = perimeter.len().try_into().unwrap();
        reg.sides = calculate_sides(&perimeter);
        regions.push(reg);
    });

    println!("Num regions: {}", regions.len());
    println!("Regions area: {:?}", regions.iter().map(|r| r.area()).collect::<Vec<u64>>());
    println!("Regions perimeter: {:?}", regions.iter().map(|r| r.perimeter).collect::<Vec<u64>>());
    println!("Part 1, {}", regions.iter().map(|r| r.perimeter * r.area()).sum::<u64>());
    println!("Part 2, {}", regions.iter().map(|r| r.sides * r.area()).sum::<u64>());
}

fn calculate_sides(perimeter: &HashSet<Fence>) -> u64 {
    let mut counted: HashSet<Fence> = HashSet::new();
    let mut sides = 0;
    for fence in perimeter.iter() {
        if !counted.insert(fence.clone()) {
            continue;
        }
        let horizontal = fence.p1.y == fence.p2.y;
        if horizontal {
            check_side(fence.up(), Direction::Up, perimeter, &mut counted);
            check_side(fence.down(), Direction::Down, perimeter, &mut counted);
        } else {
            check_side(fence.left(), Direction::Left, perimeter, &mut counted);
            check_side(fence.right(), Direction::Right, perimeter, &mut counted);
        }
        sides += 1;
    }
    sides
}

fn check_side(fence: Fence, dir: Direction, perimeter: &HashSet<Fence>, counted: &mut HashSet<Fence>) {
    if perimeter.contains(&fence) {
        let (cross1, _) = fence.cross(dir.invert());
        if perimeter.contains(&cross1) {
            return;
        }
        if counted.insert(fence.clone()) {
            check_side(fence.offset_dir(dir), dir, perimeter, counted);
        }
    }
}

fn flood(c: char, pos: Position, reg: &mut Region, grid: &Grid<char>, visited: &mut Grid<u32>, perimeter: &mut HashSet<Fence>) {
    if grid.get_pos(pos).unwrap_or('.') == c && visited.get_pos(pos).unwrap_or(20000) == 0 {
        reg.add(pos);
        visited.set_pos(reg.id, pos);

        check_neighbour(c, pos, pos + Position {x: -1, y: 0}, reg, grid, visited, perimeter);
        check_neighbour(c, pos, pos + Position {x: 1, y: 0}, reg, grid, visited, perimeter);
        check_neighbour(c, pos, pos + Position {x: 0, y: 1}, reg, grid, visited, perimeter);
        check_neighbour(c, pos, pos + Position {x: 0, y: -1}, reg, grid, visited, perimeter);

    }
}

fn check_neighbour(c: char, pos: Position, pos2: Position, reg: &mut Region, grid: &Grid<char>, visited: &mut Grid<u32>, perimeter: &mut HashSet<Fence>) {
    let fence = Fence { p1: pos, p2: pos2 };
    let pos2_region_id = visited.get_pos(pos2).unwrap_or(20000);
    if pos2_region_id != reg.id {
        perimeter.insert(fence);
        flood(c, pos2, reg, grid, visited, perimeter);
    } else {
        // Within same region
        perimeter.remove(&fence);
    }
    
}