use std::fmt;
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Position<T>(pub T, pub T);
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct Offset<T>(pub T, pub T);

impl<T> Offset<T> {
    pub fn into_type<T2>(self) -> Option<Offset<T2>>
    where
        T2: TryFrom<T>,
    {
        Some(Offset(
            T2::try_from(self.0).ok()?,
            T2::try_from(self.1).ok()?,
        ))
    }
}
impl<T> Position<T> {
    pub fn into_type<T2>(self) -> Option<Position<T2>>
    where
        T2: TryFrom<T>,
    {
        Some(Position(
            T2::try_from(self.0).ok()?,
            T2::try_from(self.1).ok()?,
        ))
    }
}

impl<T> Add for Offset<T>
where
    T: Add<T, Output = T>,
{
    type Output = Offset<T>;
    fn add(self, rhs: Self) -> Self::Output {
        Offset(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T> Sub for Offset<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Offset<T>;
    fn sub(self, rhs: Self) -> Self::Output {
        Offset(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl<T> Mul<T> for Offset<T>
where
    T: Mul<T, Output = T> + Copy,
{
    type Output = Offset<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Offset(self.0 * rhs, self.1 * rhs)
    }
}
impl<T> Add<Position<T>> for Offset<T>
where
    T: Add<T, Output = T>,
{
    type Output = Position<T>;
    fn add(self, rhs: Position<T>) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T> Sub<Position<T>> for Offset<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Position<T>;
    fn sub(self, rhs: Position<T>) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl<T> Add<Offset<T>> for Position<T>
where
    T: Add<T, Output = T>,
{
    type Output = Position<T>;
    fn add(self, rhs: Offset<T>) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl<T> Sub<Offset<T>> for Position<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Position<T>;
    fn sub(self, rhs: Offset<T>) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}
impl<T> Sub for Position<T>
where
    T: Sub<T, Output = T>,
{
    type Output = Offset<T>;
    fn sub(self, rhs: Position<T>) -> Self::Output {
        Offset(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => '^',
                Direction::Right => '>',
                Direction::Down => 'v',
                Direction::Left => '<',
            }
        )
    }
}
impl Direction {
    pub fn all_dirs() -> [Direction; 4] {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
    }
}

impl From<Direction> for Offset<i8> {
    fn from(val: Direction) -> Self {
        match val {
            Direction::Up => Offset(0, -1),
            Direction::Right => Offset(1, 0),
            Direction::Down => Offset(0, 1),
            Direction::Left => Offset(-1, 0),
        }
    }
}

impl<T> From<(T, T)> for Offset<T> {
    fn from(val: (T, T)) -> Self {
        Offset(val.0, val.1)
    }
}
impl<T> From<(T, T)> for Position<T> {
    fn from(val: (T, T)) -> Self {
        Position(val.0, val.1)
    }
}
