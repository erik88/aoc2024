use aoc2024::{file, l2d::grid::Grid};

// ---

fn main() {
    let numbers = file::digits_from_file("test.txt");
    let num_grid = Grid::new(numbers);

    println!("Part 1: {}", num_grid.get(2,2).unwrap());
}