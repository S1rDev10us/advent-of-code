use clap::Parser;
use std::fmt;
use std::fmt::Display;
use std::slice::Iter;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    actual_input: bool,
    #[arg(long)]
    star_2: bool,
}

pub fn initialize<'a>(actual_input: &'static str, test_input: &'a str) -> (&'a str, bool, bool) {
    let args = Args::parse();

    let input = if args.actual_input {
        actual_input
    } else {
        test_input
    };

    (input, args.star_2, args.actual_input)
}

#[macro_export]
macro_rules! initialize_macro {
    ($test_input:tt) => {{
        use advent_of_code::initialize_macro;
        initialize_macro!(env!("CARGO_BIN_NAME"), $test_input)
    }};
    ($path:expr, $test_input:tt) => {{
        use advent_of_code::initialize;
        initialize(
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/puzzles/",
                $path,
                "/input"
            )),
            $test_input,
        )
    }};
}

pub fn to_grid<T>(input: &str, tile_mapper: impl Fn(char) -> T) -> Grid<T>
where
    T: Display,
{
    Grid::new(
        input
            .split("\n")
            .map(|row| row.trim())
            .filter(|row| row != &"")
            .map(|row| row.chars().map(&tile_mapper).collect::<Vec<_>>())
            .collect::<Vec<_>>(),
    )
}

pub struct Grid<T>(Vec<Vec<T>>)
where
    T: Display;

impl<T> Grid<T>
where
    T: Display,
{
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
    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.0.iter()
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
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
            self.iter()
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
