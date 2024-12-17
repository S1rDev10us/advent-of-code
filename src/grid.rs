use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

fn inner_to_grid(input: &str) -> impl Iterator<Item: Iterator<Item = char>> + use<'_> {
    input
        .split("\n")
        .map(|row| row.trim())
        .filter(|row| row != &"")
        .map(|row| row.chars())
}

pub fn to_grid_chars(input: &str) -> Grid<char> {
    Grid::new(inner_to_grid(input).map(|row| row.collect()).collect())
}

pub fn to_grid<T>(input: &str, tile_mapper: impl Fn(char) -> T) -> Grid<T> {
    Grid::new(
        inner_to_grid(input)
            .map(|row| row.map(&tile_mapper).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

pub fn to_grid_parse<T: FromStr>(input: &str) -> Grid<T>
where
    <T as FromStr>::Err: Debug,
{
    to_grid(input, |tile| tile.to_string().parse::<T>().unwrap())
}

pub struct Grid<T>(Vec<Vec<T>>);

impl<T> Grid<T> {
    pub const fn new(grid: Vec<Vec<T>>) -> Grid<T> {
        Grid(grid)
    }

    pub fn contains_point(&self, pos: (usize, usize)) -> bool {
        pos.1 < self.0.len() && pos.0 < self.0[pos.1].len()
    }
    pub fn contains_signed_point(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0 && pos.1 >= 0 && self.contains_point((pos.0 as usize, pos.1 as usize))
    }
    pub fn get_pos(&self, pos: (usize, usize)) -> Option<&T> {
        if !self.contains_point(pos) {
            return None;
        }
        Some(&self.0[pos.1][pos.0])
    }
    pub fn get_signed_pos(&self, pos: (isize, isize)) -> Option<&T> {
        if !self.contains_signed_point(pos) {
            return None;
        }
        Some(&self.0[pos.1 as usize][pos.0 as usize])
    }
    pub fn set_pos(&mut self, pos: (usize, usize), tile: T) -> bool {
        if !self.contains_point(pos) {
            return false;
        }

        self.0[pos.1][pos.0] = tile;
        true
    }
    pub fn set_signed_pos(&mut self, pos: (isize, isize), tile: T) -> bool {
        if !self.contains_signed_point(pos) {
            return false;
        }

        self.0[pos.1 as usize][pos.0 as usize] = tile;
        true
    }

    /// Returns an iterator over every item in the Grid, with the position
    pub fn iter(&self) -> impl Iterator<Item = (&T, (usize, usize))> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, val)| (val, (x, y))))
    }
    /// Returns an iterator over the rows
    pub fn iter_2d(&self) -> impl Iterator<Item = &Vec<T>> {
        self.0.iter()
    }

    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }

    pub fn adjacent(&self, pos: (usize, usize)) -> Vec<(&T, (usize, usize))> {
        let mut adjacent_positions = vec![];
        for &offset in [(0, 1), (0, -1), (1, 0), (-1, 0)].iter() {
            let new_pos = (pos.0 as isize + offset.0, pos.1 as isize + offset.1);
            if self.contains_signed_point(new_pos) {
                adjacent_positions.push((
                    self.get_signed_pos(new_pos).unwrap(),
                    (new_pos.0 as usize, new_pos.1 as usize),
                ));
            }
        }
        adjacent_positions
    }
    pub fn adjacent_diagonal(&self, pos: (usize, usize)) -> Vec<(&T, (usize, usize))> {
        let mut adjacent_positions = vec![];
        for x_off in [-1, 0, 1].into_iter() {
            for y_off in [-1, 0, 1].into_iter() {
                if x_off == 0 && y_off == 0 {
                    continue;
                }
                let new_pos = (pos.0 as isize + x_off, pos.1 as isize + y_off);
                if self.contains_signed_point(new_pos) {
                    adjacent_positions.push((
                        self.get_signed_pos(new_pos).unwrap(),
                        (new_pos.0 as usize, new_pos.1 as usize),
                    ));
                }
            }
        }
        adjacent_positions
    }
}

impl<T, TIter> FromIterator<TIter> for Grid<T>
where
    TIter: IntoIterator<Item = T>,
{
    fn from_iter<I: IntoIterator<Item = TIter>>(iter: I) -> Self {
        Grid::new(
            iter.into_iter()
                .map(|row| row.into_iter().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        )
    }
}
impl<T> IntoIterator for Grid<T> {
    type Item = Vec<T>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<T> Display for Grid<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|row| row
                    .iter()
                    .map(|tile| format!("{}", tile))
                    .collect::<Vec<_>>()
                    .join(""))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}
