enum Pattern {
    Right,
    Left,
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Color {
        Color {r, g, b}
    }
}

pub struct State {
    pattern: Pattern,
    color: Color,
}

impl State {
    pub fn color(&self) -> &Color {
        &self.color
    }
}

impl State {
    fn new(pattern: Pattern, color: Color) -> State {
        State { pattern, color }
    }
}

pub struct Behavior {
    indexed_conditions: Vec<State>
}

pub struct Scene {
    behavior: Behavior,
    ant: Ant,
    field: Field,
    loop_count: u32,
}

impl Scene {
    pub fn init(x: u32, y: u32) -> Scene {
        let behavior = Behavior { indexed_conditions: vec![
            State::new(Pattern::Right, Color::new(0, 0, 0)),
            State::new(Pattern::Left, Color::new(255, 255, 255)),
        ] };
        Scene {
            behavior,
            ant: Ant {
                position: Position {
                    y: YPositionValue(LoopValue::new(
                        (y / 2).try_into().unwrap(),
                        y as i32,
                    )),
                    x: XPositionValue(LoopValue::new(
                        (x / 2).try_into().unwrap(),
                        y as i32,
                    )),
                },
                direction: Direction::Down,
            },
            field: vec![vec![0; x as usize]; y as usize],
            loop_count: 0,
        }
    }

    pub fn work(&mut self) {
        self.ant.work(&mut self.field, &self.behavior);
        self.loop_count += 1;
    }

    pub fn field(&self) -> &Field {
        &self.field
    }
    
    pub fn indexed_conditions(&self) -> &Vec<State> {
        &self.behavior.indexed_conditions
    }

    pub fn ant_position(&self) -> (i32, i32) {
        self.ant.position.value()
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
    fn work(&mut self, field: &mut Field, behavior: &Behavior) {
        let cell = &field[self.position.y_usize()][self.position.x_usize()];
        let condition = &behavior.indexed_conditions[*cell];
        match condition.pattern {
            Pattern::Right => {
                self.direction = self.direction.rotate_cw();
            }
            Pattern::Left => {
                self.direction = self.direction.rotate_ccw();
            }
        }

        field[self.position.y_usize()][self.position.x_usize()] = (cell + 1) % behavior.indexed_conditions.len();

        self.position += self.direction.vector();
    }
}

pub type Field = Vec<Vec<usize>>;

use std::ops;

#[derive(Clone, Debug, PartialEq)]
pub struct LoopValue {
    value: i32,
    max_value: i32,
}

impl LoopValue {
    fn new(value: i32, max_value: i32) -> LoopValue {
        Self { value, max_value }
    }

    fn value(&self) -> i32 {
        self.value
    }
}

impl ops::AddAssign<i32> for LoopValue {
    fn add_assign(&mut self, rhs: i32) {
        self.value = (self.value + rhs).rem_euclid(self.max_value);
    }
}

impl Into<i32> for LoopValue {
    fn into(self) -> i32 {
        self.value
    }
}

impl Into<usize> for LoopValue {
    fn into(self) -> usize {
        self.value as usize
    }
}

#[cfg(test)]
mod tests {
    use crate::LoopValue;

    #[test]
    fn add_assign_works() {
        let mut x = LoopValue::new(0, 10);
        x += 1;
        assert_eq!(x.value(), 1);
    }

    #[test]
    fn add_assign_positive_overflow_works() {
        let mut x = LoopValue::new(9, 10);
        x += 1;
        assert_eq!(x.value(), 0);
    }

    #[test]
    fn add_assign_positive_negative_overflow_works() {
        let mut x = LoopValue::new(0, 10);
        x += -1;
        assert_eq!(x.value(), 9);
    }
}

#[derive(Clone)]
struct YPositionValue(LoopValue);
impl YPositionValue {
    fn value(&self) -> i32 {
        self.0.value()
    }
}
impl ops::AddAssign<YVectorValue> for YPositionValue {
    fn add_assign(&mut self, rhs: YVectorValue) {
        self.0 += rhs.0;
    }
}
impl Into<i32> for YPositionValue {
    fn into(self) -> i32 {
        self.0.into()
    }
}

#[derive(Clone)]
struct XPositionValue(LoopValue);
impl XPositionValue {
    fn value(&self) -> i32 {
        self.0.value()
    }
}
impl ops::AddAssign<XVectorValue> for XPositionValue {
    fn add_assign(&mut self, rhs: XVectorValue) {
        self.0 += rhs.0;
    }
}
impl Into<i32> for XPositionValue {
    fn into(self) -> i32 {
        self.0.into()
    }
}

#[derive(Clone)]
pub struct YVectorValue(i32);

#[derive(Clone)]
pub struct XVectorValue(i32);

#[derive(Clone)]
pub struct Position {
    y: YPositionValue,
    x: XPositionValue,
}
impl Position {
    fn y_usize(&self) -> usize {
        self.y.value() as usize
    }
    fn x_usize(&self) -> usize {
        self.x.value() as usize
    }
    fn value(&self) -> (i32, i32) {
        (self.x.value(), self.y.value())
    }
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
