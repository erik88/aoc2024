use aoc2024::{file, l2d::{direction::Direction, grid::Grid, position::Position}};

// ---

#[derive(Copy, Clone, PartialEq, Debug)]
enum Item {
    Box,
    Wall,
    Space
}

fn main() {
    let lines = file::lines_from_file("input.txt");
    let (mut grid, instructions, mut robot_pos) = parse_lines(lines);

    for dir in instructions {
        let (behind_pos, behind_item) = get_position_through_boxes(&grid, robot_pos, dir);
        if behind_item == Item::Space {
            // Shift all boxes once
            // If there are none, behind_pos and robot_pos + dir should be the same.
            *grid.get_mut_pos(behind_pos).unwrap() = Item::Box;
            robot_pos = robot_pos + dir.pos();
            *grid.get_mut_pos(robot_pos).unwrap() = Item::Space;
            
        }
    }
    // println!("Robot pos: {}", robot_pos);
    // println!("Directions: {:?}", instructions);
    // println!("Grid-test: {:?} {:?}", grid.get(3, 1), grid.get(4,1));
    let mut sum: i64 = 0;
    grid.for_each_mut(|itm, pos| {
        if itm == Item::Box {
            sum += 100*pos.y + pos.x;
        }
    });

    println!("Part 1, {}", sum);
}

fn get_position_through_boxes(grid: &Grid<Item>, robot_pos: Position, dir: Direction) -> (Position, Item) {
    let mut p: Position = robot_pos;
    loop {
        p = p + dir.pos();
        let itm = grid.get_pos(p).unwrap();
        match itm {
            Item::Box => continue,
            Item::Wall | Item::Space => return (p, itm),
        }
    }
}

fn parse_lines(lines: Vec<String>) -> (Grid<Item>, Vec<Direction>, Position) {
    let mut split_index = 0;
    for (index, line) in lines.iter().enumerate() {
        if line.is_empty() {
            split_index = index;
            break;
        }
    }

    let mut robot_pos = Position { x:0 , y:0 };
    let (grid_lines, dir_lines) = lines.split_at(split_index + 1);
    let mut line_count = -1;
    let grid_items: Vec<Vec<Item>> = grid_lines.into_iter().flat_map(|l| {
        let mut char_count = -1;
        line_count += 1;
        if l.is_empty() {
            None
        } else { 
            Some(l.chars().map(|c| {
                char_count += 1;
                match c {
                    '#' => Item::Wall,
                    'O' => Item::Box,
                    '.' => Item::Space,
                    '@' => {
                        robot_pos = Position { x: char_count, y: line_count };
                        Item::Space
                    },
                    _ => panic!("Unrecognized grid character '{}'", c)
                }
            }).collect())
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

