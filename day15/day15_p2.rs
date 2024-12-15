use std::collections::VecDeque;

use aoc2024::{file, l2d::{direction::Direction, grid::Grid, position::Position}};

// ---

#[derive(Copy, Clone, PartialEq, Debug)]
enum Item {
    Box(usize),
    Wall,
    Space
}

fn main() {
    let lines = file::lines_from_file("input.txt");
    let (mut grid, instructions, mut robot_pos) = parse_lines(lines);
    // let mut count = 0;
    //  println!("{}",print_grid(&grid, robot_pos));

    for dir in instructions {
        let mut marked: Vec<(usize, Position)> = Vec::new();
        let mut to_check: VecDeque<(Position, Direction)> = VecDeque::new();
        to_check.push_back((robot_pos + dir.pos(), dir));
        let mut hinder = false;
        while to_check.len() > 0 {
            if !push_mark(&grid, &mut marked, &mut to_check) {
                hinder = true;
                break;
            }

        }
        if !hinder {
            robot_pos = robot_pos + dir.pos();

            marked.clone().into_iter().rev().for_each(|(id, b_pos)| {
                // println!("id {} is marked on {}", id, b_pos);
                *grid.get_mut_pos(b_pos).unwrap() = Item::Space;
                *grid.get_mut_pos(b_pos + Position { x: 1, y: 0 }).unwrap() = Item::Space;
                *grid.get_mut_pos(b_pos + dir.pos()).unwrap() = Item::Box(id);
                *grid.get_mut_pos(b_pos + dir.pos() + Position { x: 1, y: 0 }).unwrap() = Item::Box(id);
                // println!("Removing from {} {}", b_pos, b_pos + Position { x: 1, y: 0 });
                // println!("Adding to {} {}", b_pos + dir.pos(), b_pos + dir.pos() + Position { x: 1, y: 0 });
            });
        }
        // if count < 30 {
        //     println!("{:?}\n{}",dir, print_grid(&grid, robot_pos));
        //     count += 1;
        // }
        // if count < 1 {
        //     println!("{:?}",marked);
        // } else {
        //     break;
        // }
    }
    // println!("Robot pos: {}", robot_pos);
    // println!("Directions: {:?}", instructions);
    // println!("Grid-test: {:?} {:?}", grid.get(3, 1), grid.get(4,1));

    // println!("{}",print_grid(&grid, robot_pos));

    let mut sum: i64 = 0;
    let mut in_box = false;
    grid.for_each_mut(|itm, pos| {
        match itm {
            Item::Box(_) => {
                if !in_box {
                    in_box = true;
                // Very unclear example...
                // let x = cmp::min(pos.x, grid.width as i64 - pos.x + 2);
                // let y = cmp::min(pos.y, grid.height as i64 - pos.y + 1);
                let x = pos.x;
                let y = pos.y;
                sum += 100*y + x;
                } else {
                    in_box = false;
                }
            }
            Item::Wall | Item::Space => (),
        }
    });

    // let s = print_grid(&grid, robot_pos);
    // println!("{}",s);

    // Too low 1553413
    println!("Part 2, {}", sum);
}

// fn zero_pad(i: usize) -> String {
//     if i < 10 {
//         let s1 = String::from_str("0").unwrap();
//         let s2 = i.to_string();
//         return s1 + &s2
//     }
//     i.to_string()
// }

fn push_mark(grid: &Grid<Item>, marked: &mut Vec<(usize, Position)>, to_check: &mut VecDeque<(Position, Direction)>) -> bool {
    let new_pos_maybe = to_check.pop_front();
    if new_pos_maybe.is_none() {
        return true;
    }
    let (new_pos, dir) = new_pos_maybe.unwrap();
    let itm = grid.get_pos(new_pos).unwrap();
    match itm {
        Item::Box(id) => {
            if marked.iter().find(|(m_id, _)| *m_id == id).is_none() {
                let left_pos = new_pos + Position { x: -1, y: 0};
                let left_itm = grid.get_pos(left_pos).unwrap();
                let mut box_pos = new_pos;
                // println!("Left pos: {}, Left item {:?}", left_pos, left_itm);
                if let Item::Box(left_box_id) = left_itm {
                    if left_box_id == id {
                        box_pos = left_pos;
                        // println!("Is left pos.");
                    }
                }
                // println!("Pushing to marked {}", box_pos);
                marked.push((id, box_pos));

                match dir {
                    Direction::Left => to_check.push_back((box_pos + dir.pos(), dir)),
                    Direction::Right => to_check.push_back((box_pos+(dir.pos()*2), dir)),
                    Direction::Up | Direction::Down => {
                        to_check.push_back((box_pos + dir.pos(), dir));
                        to_check.push_back((box_pos+ Position { x: 1, y: 0 } + dir.pos(), dir));
                    }
                }
                return true;
            } else {
                // A box that we have already seen. Ignore and await result from before.
                return true;
            }
        },
        Item::Wall => return false,
        Item::Space => return true,
    };
}

fn parse_lines(lines: Vec<String>) -> (Grid<Item>, Vec<Direction>, Position) {
    let mut split_index = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            split_index = index;
            break;
        }
    }

    let mut box_count = 0;
    let mut robot_pos = Position { x:0 , y:0 };
    let (grid_lines, dir_lines) = lines.split_at(split_index + 1);
    let mut line_count = -1;
    let grid_items: Vec<Vec<Item>> = grid_lines.into_iter().flat_map(|l| {
        line_count += 1;
        if l.is_empty() {
            None
        } else { 
            let mut item_row: Vec<Item> = Vec::new();
            let mut char_count = -2;
            for c in l.chars() {
                char_count += 2;
                match c {
                    '#' => {
                        item_row.push(Item::Wall);
                        item_row.push(Item::Wall);
                    },
                    'O' => {
                        item_row.push(Item::Box(box_count));
                        item_row.push(Item::Box(box_count));
                        box_count += 1;
                    }
                    '.' => {
                        item_row.push(Item::Space);
                        item_row.push(Item::Space);
                    },
                    '@' => {
                        item_row.push(Item::Space);
                        item_row.push(Item::Space);
                        robot_pos = Position { x: char_count, y: line_count };
                    },
                    _ => panic!("Unrecognized grid character '{}'", c)
                }
            }
            Some(item_row)
        }
    }).collect();

    let dirs: Vec<Direction> = dir_lines.join("").chars().map(|c| match c {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Unexpected direction character '{}'", c)
    }).collect();

    (Grid::new(grid_items), dirs, robot_pos)
}

// fn print_grid(grid: &Grid<Item>, player_pos: Position) -> String {
//     let mut s = String::new();
//     let mut y = 0;
//     let mut curr_box: Option<usize> =  None;
//     grid.for_each_mut(|itm, pos| {
//         // if pos.y == 4 {
//         //     println!("{}", pos);
//         //     if let Item::Box(id) = itm {
//         //         if id == 8 {
//         //             println!("Control: {:?}", itm);
//         //         }
//         //     }
//         // }

//         if pos == player_pos {
//             s += "@";
//             return;
//         }
//         if pos.y != y {
//             y = pos.y;
//             s += "\n";
//         }

//         match itm {
//             Item::Box(id) => {
//                 let fafa: Vec<char> = zero_pad(id).chars().collect();
//                 if curr_box == None {
//                     s.push(fafa[0]);
//                     curr_box = Some(id)
//                 } else {
//                     s.push(fafa[1]);
//                     curr_box = None
//                 }
//             },
//             Item::Wall => s += "#",
//             Item::Space => s += ".",
//         }
//     });
//     s
// }