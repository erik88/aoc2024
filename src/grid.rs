use crate::position::Position;

#[derive(Clone)]
pub struct Grid<T: Copy+PartialEq> {
    points: Vec<Vec<T>>,
}

impl<T: Copy+PartialEq> Grid<T> {
    pub fn new(data: Vec<Vec<T>>) -> Grid<T> {
        Grid {
            points: data,
        }
    }

    pub fn get(&self, x: i32, y:i32) -> Option<T> {
        let xi: usize = x.try_into().ok()?;
        let yi: usize = y.try_into().ok()?;

        self.points.get(yi)?.get(xi).map(|t| *t)
    }

    pub fn get_mut(&mut self, x: i32, y:i32) -> Option<&mut T> {
        let xi: usize = x.try_into().ok()?;
        let yi: usize = y.try_into().ok()?;

        self.points.get_mut(yi)?.get_mut(xi)
    }

    pub fn map<V: Copy+PartialEq>(&self, f: fn(T) -> V) -> Grid<V> {
        return Grid::new(
            self.points.iter().map(|row| row.iter().map(|t| f(*t)).collect()).collect()
        );
    }

    pub fn find_first(&self, t: T) -> Option<Position> {
        for (yi, row) in self.points.iter().enumerate() {
            for (xi, itm) in row.iter().enumerate() {
                if *itm == t {
                    let x: i32 = xi.try_into().ok()?;
                    let y: i32 = yi.try_into().ok()?;
                    return Some(Position { x, y })
                }
            }
        }
        None
    }
}