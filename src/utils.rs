use std::ops::Add;

#[derive(Clone, Copy)]
pub struct Size {
    pub w: i32,
    pub h: i32
}

#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}