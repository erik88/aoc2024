use super::position::Position;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    pub fn pos(&self) -> Position {
        match self {
            Direction::Up => Position { x: 0, y: -1 },
            Direction::Down => Position { x: 0, y: 1 },
            Direction::Left => Position { x: -1, y: 0 },
            Direction::Right => Position { x: 1, y: 0 },
        }
    }

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }

    pub fn for_each(mut f: impl FnMut(Direction) -> ()) {
        f(Direction::Right);
        f(Direction::Up);
        f(Direction::Left);
        f(Direction::Down);
    }

    pub fn all() -> [Direction; 4] {
        return [Direction::Right, Direction::Up, Direction::Left, Direction::Down];
    }
}