use std::{fmt, ops};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32
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

impl ops::Mul<Position> for i32 {
    fn mul(self, pos: Position) -> Position {
        Position {
            x: self * pos.x,
            y: self * pos.y
        }
    }
    
    type Output = Position;
}

impl ops::Mul<i32> for Position {
    fn mul(self, k: i32) -> Position {
        Position {
            x: self.x * k,
            y: self.y * k
        }
    }
    
    type Output = Position;
}