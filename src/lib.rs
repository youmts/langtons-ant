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
        Color { r, g, b }
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
    indexed_conditions: Vec<State>,
}

pub struct Scene {
    behavior: Behavior,
    ants: Vec<Ant>,
    field: Field,
    loop_count: u32,
}

fn find_behavior(number: u8) -> Behavior {
    match number {
        0 => Behavior {
            indexed_conditions: vec![
                State::new(Pattern::Right, Color::new(0, 0, 0)),
                State::new(Pattern::Left, Color::new(255, 255, 255)),
            ],
        },
        1 => Behavior {
            indexed_conditions: vec![
                State::new(Pattern::Left, Color::new(0, 0, 0)),
                State::new(Pattern::Right, Color::new(255, 0, 0)),
                State::new(Pattern::Right, Color::new(0, 255, 0)),
                State::new(Pattern::Right, Color::new(0, 0, 255)),
                State::new(Pattern::Right, Color::new(255, 255, 0)),
                State::new(Pattern::Right, Color::new(255, 0, 255)),
                State::new(Pattern::Left, Color::new(0, 255, 255)),
                State::new(Pattern::Left, Color::new(255, 255, 255)),
                State::new(Pattern::Right, Color::new(128, 128, 128)),
            ],
        },
        2 => Behavior {
            indexed_conditions: vec![
                State::new(Pattern::Left, Color::new(0, 0, 0)),
                State::new(Pattern::Left, Color::new(255, 0, 0)),
                State::new(Pattern::Right, Color::new(0, 255, 0)),
                State::new(Pattern::Right, Color::new(0, 0, 255)),
                State::new(Pattern::Right, Color::new(255, 255, 0)),
                State::new(Pattern::Left, Color::new(255, 0, 255)),
                State::new(Pattern::Right, Color::new(0, 255, 255)),
                State::new(Pattern::Left, Color::new(255, 255, 255)),
                State::new(Pattern::Right, Color::new(128, 128, 128)),
                State::new(Pattern::Left, Color::new(128, 0, 0)),
                State::new(Pattern::Left, Color::new(0, 128, 0)),
                State::new(Pattern::Right, Color::new(0, 0, 128)),
            ],
        },
        3 => Behavior {
            indexed_conditions: vec![
                State::new(Pattern::Right, Color::new(0, 0, 0)),
                State::new(Pattern::Right, Color::new(255, 0, 0)),
                State::new(Pattern::Left, Color::new(0, 255, 0)),
                State::new(Pattern::Left, Color::new(0, 0, 255)),
                State::new(Pattern::Left, Color::new(255, 255, 0)),
                State::new(Pattern::Right, Color::new(255, 0, 255)),
                State::new(Pattern::Left, Color::new(0, 255, 255)),
                State::new(Pattern::Left, Color::new(255, 255, 255)),
                State::new(Pattern::Left, Color::new(128, 128, 128)),
                State::new(Pattern::Right, Color::new(128, 0, 0)),
                State::new(Pattern::Right, Color::new(0, 128, 0)),
                State::new(Pattern::Right, Color::new(0, 0, 128)),
            ],
        },
        _default => panic!(),
    }
}

fn find_ants(number: u8, x: u32, y: u32) -> Vec<Ant> {
    let x = x as i32;
    let y = y as i32;
    match number {
        1 => vec![Ant {
            position: Position {
                y: YPositionValue(LoopValue::new(y / 2, y)),
                x: XPositionValue(LoopValue::new(x / 2, y)),
            },
            direction: Direction::Down,
        }],
        2 => vec![
            Ant {
                position: Position {
                    y: YPositionValue(LoopValue::new(y / 2, y)),
                    x: XPositionValue(LoopValue::new(x / 3, y)),
                },
                direction: Direction::Down,
            },
            Ant {
                position: Position {
                    y: YPositionValue(LoopValue::new(y / 2, y)),
                    x: XPositionValue(LoopValue::new(x / 3 * 2, y)),
                },
                direction: Direction::Down,
            },
        ],
        3 => vec![
            Ant {
                position: Position {
                    y: YPositionValue(LoopValue::new(y / 2, y)),
                    x: XPositionValue(LoopValue::new(x / 4, y)),
                },
                direction: Direction::Down,
            },
            Ant {
                position: Position {
                    y: YPositionValue(LoopValue::new(y / 2, y)),
                    x: XPositionValue(LoopValue::new(x / 2, y)),
                },
                direction: Direction::Down,
            },
            Ant {
                position: Position {
                    y: YPositionValue(LoopValue::new(y / 2, y)),
                    x: XPositionValue(LoopValue::new(x / 4 * 3, y)),
                },
                direction: Direction::Down,
            },
        ],
        _default => panic!(),
    }
}

impl Scene {
    pub fn init(x: u32, y: u32) -> Scene {
        Scene {
            behavior: find_behavior(0),
            ants: find_ants(3, x, y),
            field: vec![vec![0; x as usize]; y as usize],
            loop_count: 0,
        }
    }

    pub fn work(&mut self) {
        for ant in &mut self.ants {
            ant.work(&mut self.field, &self.behavior);
        }
        self.loop_count += 1;
    }

    pub fn field(&self) -> &Field {
        &self.field
    }

    pub fn indexed_conditions(&self) -> &Vec<State> {
        &self.behavior.indexed_conditions
    }

    pub fn ants(&self) -> &Vec<Ant> {
        &self.ants
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

        field[self.position.y_usize()][self.position.x_usize()] =
            (cell + 1) % behavior.indexed_conditions.len();

        self.position += self.direction.vector();
    }

    pub fn position(&self) -> (i32, i32) {
        (self.position.x.value(), self.position.y.value())
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
