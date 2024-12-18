use std::{cmp::Ordering, fmt, ops};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Position {
    pub x: i64,
    pub y: i64
}

impl Position {
    pub fn up(&self) -> Position {
        return Position { x: self.x, y: self.y -1 }
    }
    pub fn down(&self) -> Position {
        return Position { x: self.x, y: self.y +1 }
    }
    pub fn left(&self) -> Position {
        return Position { x: self.x-1, y: self.y }
    }
    pub fn right(&self) -> Position {
        return Position { x: self.x+1, y: self.y }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl ops::Add<Position> for Position {
    fn add(self, p: Position) -> Position {
        Position {
            x: self.x + p.x,
            y: self.y + p.y
        }
    }
    
    type Output = Position;
}

impl ops::Sub<Position> for Position {
    fn sub(self, p: Position) -> Position {
        Position {
            x: self.x - p.x,
            y: self.y - p.y
        }
    }
    
    type Output = Position;
}

impl ops::Mul<Position> for i64 {
    fn mul(self, pos: Position) -> Position {
        Position {
            x: self * pos.x,
            y: self * pos.y
        }
    }
    
    type Output = Position;
}

impl ops::Mul<i64> for Position {
    fn mul(self, k: i64) -> Position {
        Position {
            x: self.x * k,
            y: self.y * k
        }
    }
    
    type Output = Position;
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y > other.y {
            Ordering::Greater
        } else if self.y == other.y {
            if self.x > other.x {
                return Ordering::Greater
            } if self.x == other.x {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}