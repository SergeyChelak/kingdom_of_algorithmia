pub mod strings;

pub type Vec2<T> = Vec<Vec<T>>;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Position2<T> {
    pub row: T,
    pub col: T,
}

impl<T> Position2<T> {
    pub fn new(row: T, col: T) -> Self {
        Self { row, col }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn _all() -> [Direction; 4] {
        use Direction::*;
        [Up, Down, Left, Right]
    }

    pub fn all_clockwise() -> [Direction; 4] {
        [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ]
    }
}
