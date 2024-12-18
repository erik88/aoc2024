use super::position::Position;

#[derive(Clone)]
pub struct Grid<T: Copy+PartialEq> {
    points: Vec<Vec<T>>,
    pub width: u32,
    pub height: u32
}

impl Grid<char> {
    pub fn print(&self) {
        for p in &self.points {
            let s: String = p.iter().collect();
            println!("{}", s);
        }
    }
}

impl<T: Copy+PartialEq> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Grid<T> {
        Grid {
            width: data.get(0).unwrap().len() as u32,
            height: data.len() as u32,
            points: data,
        }
    }

    pub fn from(item: T, width: u32, height: u32) -> Grid<T> {
        let mut rows = Vec::new();
        for _ in 0..height {
            rows.push(vec![item; width.try_into().unwrap()]);
        }
        Grid {
            width,
            height,
            points: rows,
        }
    }

    pub fn get(&self, x: i32, y:i32) -> Option<T> {
        let xi: usize = x.try_into().ok()?;
        let yi: usize = y.try_into().ok()?;

        self.points.get(yi)?.get(xi).map(|t| *t)
    }

    pub fn get_pos(&self, pos: Position) -> Option<T> {
        let xi: usize = pos.x.try_into().ok()?;
        let yi: usize = pos.y.try_into().ok()?;

        self.points.get(yi)?.get(xi).map(|t| *t)
    }

    pub fn get_mut(&mut self, x: i64, y:i64) -> Option<&mut T> {
        let xi: usize = x.try_into().ok()?;
        let yi: usize = y.try_into().ok()?;

        self.points.get_mut(yi)?.get_mut(xi)
    }


    pub fn get_mut_pos(&mut self, p: Position) -> Option<&mut T> {
        self.get_mut(p.x, p.y)
    }

    pub fn set_pos(&mut self, t: T, p: Position) {
        let itm = self.get_mut_pos(p).unwrap();
        *itm = t;
    }

    pub fn try_set_pos(&mut self, t: T, p: Position) -> bool {
        if let Some(itm) = self.get_mut_pos(p) {
            *itm = t;
            return true;
        }
        return false;
    }

    pub fn map<V: Copy+PartialEq>(&self, f: fn(T) -> V) -> Grid<V> {
        return Grid::new(
            self.points.iter().map(|row| row.iter().map(|t| f(*t)).collect()).collect()
        );
    }

    pub fn for_each_mut(&self, mut f: impl FnMut(T, Position) -> ()) {
        for (y, row) in self.points.iter().enumerate() {
            for (x, itm) in row.iter().enumerate() {
                f(*itm, Position {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap()
                })
            }
        }
    }

    pub fn for_each(&self, f: impl Fn(T, Position) -> ()) {
        for (y, row) in self.points.iter().enumerate() {
            for (x, itm) in row.iter().enumerate() {
                f(*itm, Position {
                    x: x.try_into().unwrap(),
                    y: y.try_into().unwrap()
                })
            }
        }
    }

    pub fn find_first(&self, t: T) -> Option<Position> {
        for (yi, row) in self.points.iter().enumerate() {
            for (xi, itm) in row.iter().enumerate() {
                if *itm == t {
                    let x: i64 = xi.try_into().ok()?;
                    let y: i64 = yi.try_into().ok()?;
                    return Some(Position { x, y })
                }
            }
        }
        None
    }
}