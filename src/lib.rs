pub const FIELD_WIDTH: usize = 600;
pub const FIELD_HEIGHT: usize = 600;

pub struct Scene {
    ant: Ant,
    field: Field,
    loop_count: u32,
}

impl Scene {
    pub fn init() -> Scene {
        Scene {
            ant: Ant {
                position: Position {
                    y: YPositionValue((FIELD_HEIGHT / 2).try_into().unwrap()),
                    x: XPositionValue((FIELD_WIDTH / 2).try_into().unwrap()),
                },
                direction: Direction::Down,
            },
            field: vec![vec![Cell::Black; FIELD_WIDTH]; FIELD_HEIGHT],
            loop_count: 0,
        }
    }

    pub fn work(&mut self) {
        self.ant.work(&mut self.field);
        self.loop_count += 1;
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn loop_count(&self) -> u32 {
        self.loop_count
    }
}

pub struct Ant {
    position: Position,
    direction: Direction,
}

impl Ant {
    fn work(&mut self, field: &mut Field) {
        let cell = &field[self.position.y.0 as usize][self.position.x.0 as usize];
        match cell {
            Cell::White => {
                self.direction = self.direction.rotate_cw();
            }
            Cell::Black => {
                self.direction = self.direction.rotate_ccw();
            }
        }

        field[self.position.y.0 as usize][self.position.x.0 as usize] = cell.reverse();

        self.position += self.direction.vector();
    }
}

pub type Field = Vec<Vec<Cell>>;

use std::ops;

#[derive(Clone, Debug, PartialEq)]
pub struct YPositionValue(i32);
impl ops::AddAssign<YVectorValue> for YPositionValue {
    fn add_assign(&mut self, rhs: YVectorValue) {
        *self = Self((self.0 + rhs.0).rem_euclid(FIELD_HEIGHT as i32))
    }
}

#[cfg(test)]
mod tests {
    use crate::{YPositionValue, YVectorValue, FIELD_HEIGHT};

    #[test]
    fn add_assign_works() {
        let mut value = YPositionValue(0);
        value += YVectorValue(1);
        assert_eq!(value, YPositionValue(1));
    }

    #[test]
    fn add_assign_positive_overflow_works() {
        let mut value = YPositionValue(FIELD_HEIGHT as i32 - 1);
        value += YVectorValue(1);
        assert_eq!(value, YPositionValue(0));
    }

    #[test]
    fn add_assign_positive_negative_overflow_works() {
        let mut value = YPositionValue(0);
        value += YVectorValue(-1);
        assert_eq!(value, YPositionValue(FIELD_HEIGHT as i32 - 1));
    }
}

#[derive(Clone)]
pub struct YVectorValue(i32);

#[derive(Clone)]
pub struct XPositionValue(i32);
impl ops::AddAssign<XVectorValue> for XPositionValue {
    fn add_assign(&mut self, rhs: XVectorValue) {
        *self = Self((self.0 + rhs.0).rem_euclid(FIELD_WIDTH as i32))
    }
}

#[derive(Clone)]
pub struct XVectorValue(i32);

#[derive(Clone)]
pub struct Position {
    y: YPositionValue,
    x: XPositionValue,
}
impl ops::AddAssign<Vector> for Position {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

#[derive(Clone)]
struct Vector {
    y: YVectorValue,
    x: XVectorValue,
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate_cw(&self) -> Direction {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn rotate_ccw(&self) -> Direction {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    fn vector(&self) -> Vector {
        match self {
            Self::Up => Vector {
                y: YVectorValue(-1),
                x: XVectorValue(0),
            },
            Self::Right => Vector {
                y: YVectorValue(0),
                x: XVectorValue(1),
            },
            Self::Down => Vector {
                y: YVectorValue(1),
                x: XVectorValue(0),
            },
            Self::Left => Vector {
                y: YVectorValue(0),
                x: XVectorValue(-1),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub enum Cell {
    White,
    Black,
}

impl Cell {
    fn reverse(&self) -> Cell {
        match self {
            Cell::White => Cell::Black,
            Cell::Black => Cell::White,
        }
    }
}
