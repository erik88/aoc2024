use std::collections::{HashMap, VecDeque};

use aoc2024::{file, l2d::{grid::Grid, position::Position}};

// ---

// const CUTOFF: u64 = 50;
// const FILE_NAME: &str = "test.txt";
const CUTOFF: u64 = 100;
const FILE_NAME: &str = "input.txt";

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

    let mut cheats: u64 = 0;
    let distance_to_end: HashMap<Position, u64> = get_all_distances_to_end(&maze, end);
    for x1 in 1..maze.width-1 {
        for y1 in 1..maze.height-1 {
            let x: i64 = x1.into();
            let y: i64 = y1.into();
            if maze.get(x, y).unwrap() == '#' {
                continue;
            }
            cheats += find_cheats_from(Position { x, y }, &maze, &distance_to_end);
        }
    }
    
    // Should be for test 285
    // 1023045 is too low
    println!("Cheats {}", cheats);
}

fn find_cheats_from(start_pos: Position, maze: &Grid<char>, distance_to_end: &HashMap<Position, u64>) -> u64 {
    let mut cheats = 0;
    let start_distance_to_end = distance_to_end.get(&start_pos).unwrap();
    if maze.get_pos(start_pos).unwrap() != '.' {
        panic!("Doing for other than .")
    }
    for x in (start_pos.x - 20)..=(start_pos.x + 20) {
        for y in (start_pos.y - 20)..=(start_pos.y + 20) {
            let cheat_length: u64 = ((start_pos.x - x).abs() + (start_pos.y - y).abs()).try_into().unwrap();
            if cheat_length <= 20 {
                let end_pos = Position{x, y};
                if let Some(end_distance_to_end) = distance_to_end.get(&end_pos) {
                    if maze.get_pos(end_pos).unwrap() != '.' {
                        panic!("Endpos was not a dot");
                    }
                    if *end_distance_to_end + cheat_length + CUTOFF <= *start_distance_to_end {
                        cheats += 1;
                    }
                }
            }
        }
    }
    cheats
}

// WAIT YOU CAN RUN IN AND OUT OF WALLS!!?
// fn find_cheats_from(start_pos: Position, maze: &Grid<char>, distance_to_end: &HashMap<Position, u64>) -> u64 {
//     let mut horizon: VecDeque<(Position, u64)> = VecDeque::new();
//     let mut shortest: HashMap<Position, u64> = HashMap::new();
//     let mut ends: HashSet<Position> = HashSet::new();
//     let start_distance_to_end = distance_to_end.get(&start_pos).unwrap();
//     horizon.push_back((start_pos.right(), 1));
//     horizon.push_back((start_pos.up(), 1));
//     horizon.push_back((start_pos.left(), 1));
//     horizon.push_back((start_pos.down(), 1));

//     let mut count = 0;

//     while let Some((pos, dist)) = horizon.pop_front() {
//         match maze.get_pos(pos) {
//             None => continue,
//             Some('.') => {
//                 if ends.insert(pos) {
//                     match distance_to_end.get(&pos) {
//                         Some(end_distance_to_end) => if *end_distance_to_end + dist + CUTOFF <= *start_distance_to_end {
//                             count += 1;
//                         },
//                         None => panic!("Unmarked distance-to-end at {}", pos),
//                     }
//                 }
//                 if dist < 20 {

//                 }
//             }
//             Some('#') => {
//                 if dist >= 20 {
//                     // We have taken 20 steps and are still not outside.
//                     continue;
//                 }
//                 let mut updated = false;
//                 if let Some(short) = shortest.get_mut(&pos) {
//                     if *short > dist {
//                         *short = dist;
//                         updated = true;
//                     }
//                 } else {
//                     shortest.insert(pos, dist);
//                     updated = true;
//                 }
//                 if updated {
//                     horizon.push_back((pos.right(), dist+1));
//                     horizon.push_back((pos.up(), dist+1));
//                     horizon.push_back((pos.left(), dist+1));
//                     horizon.push_back((pos.down(), dist+1));
//                 }
//             },
//             Some(c) => panic!("Unexpected char: {}", c),
//         }
//     }

//     // panic!("End here");

//     count
// }
       


fn get_all_distances_to_end(maze: &Grid<char>, end: Position) -> HashMap<Position, u64> {
    let mut horizon: VecDeque<(Position, u64)> = VecDeque::new();
    let mut shortest: HashMap<Position, u64> = HashMap::new();
    horizon.push_back((end, 0));

    while let Some((pos, dist)) = horizon.pop_front() {
        match maze.get_pos(pos) {
            None | Some('#') => continue,
            Some('.') => {
                let mut updated = false;
                if let Some(short) = shortest.get_mut(&pos) {
                    if *short > dist {
                        *short = dist;
                        updated = true;
                    }
                } else {
                    shortest.insert(pos, dist);
                    updated = true;
                }
                if updated {
                    horizon.push_back((pos.right(), dist+1));
                    horizon.push_back((pos.up(), dist+1));
                    horizon.push_back((pos.left(), dist+1));
                    horizon.push_back((pos.down(), dist+1));
                }
            },
            Some(c) => panic!("Unexpected char: {}", c),
        }
    }

    shortest
}