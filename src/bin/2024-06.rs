use advent_of_code::initialize_macro;
use std::fmt;

#[derive(Debug, Clone)]
enum Direction {
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
    fn to_offset(&self) -> (isize, isize) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        }
    }
}

#[derive(Debug)]
enum Tile {
    Guard(Direction),
    Empty(bool),
    Obstruction,
}

struct Map(Vec<Vec<Tile>>);
impl Map {
    fn pos_in_map(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0
            && pos.1 >= 0
            && self.0.len() > pos.1 as usize
            && self.0[pos.1 as usize].len() > pos.0 as usize
    }
    fn get_pos(&self, pos: (isize, isize)) -> Option<&Tile> {
        if !self.pos_in_map(pos) {
            return None;
        }
        Some(&self.0[pos.1 as usize][pos.0 as usize])
    }
    fn set_pos(&mut self, pos: (isize, isize), tile: Tile) -> bool {
        if !self.pos_in_map(pos) {
            return false;
        }

        self.0[pos.1 as usize][pos.0 as usize] = tile;
        true
    }
    fn guards(&self) -> Vec<(usize, usize)> {
        let mut guards = vec![];
        for (y, row) in self.0.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if let Tile::Guard(_) = tile {
                    guards.push((x, y));
                }
            }
        }
        guards
    }
}

fn main() {
    let (input, _, is_actual_input) = initialize_macro!(
        "2024/06",
        "\
            ....#.....\n\
            .........#\n\
            ..........\n\
            ..#.......\n\
            .......#..\n\
            ..........\n\
            .#..^.....\n\
            ........#.\n\
            #.........\n\
            ......#...\
        "
    );

    let mut map = Map(input
        .split('\n')
        .filter(|row| row.trim() != "")
        .map(|row| {
            row.chars()
                .map(|tile| match tile {
                    '#' => Tile::Obstruction,
                    '^' => Tile::Guard(Direction::Up),
                    '.' => Tile::Empty(false),
                    _ => unreachable!("Tile '{:?}' does not mean anything", tile),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>());

    loop {
        let guards = map.guards();
        if guards.is_empty() {
            break;
        }
        for guard in guards {
            move_guard(&mut map, guard);
        }
        if !is_actual_input {
            println!(
                "{:}\n",
                map.0
                    .iter()
                    .map(|row| row
                        .iter()
                        .map(|tile| match tile {
                            Tile::Empty(true) => String::from("X"),
                            Tile::Empty(false) => String::from("."),
                            Tile::Guard(dir) => format!("{}", dir),
                            Tile::Obstruction => String::from("#"),
                        })
                        .collect::<Vec<_>>()
                        .join(""))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }

    let output_1 = map
        .0
        .iter()
        .flatten()
        .filter(|tile| matches!(tile, Tile::Empty(true)))
        .count();
    dbg!(output_1);
    if is_actual_input {
        assert_eq!(4433, output_1);
    } else {
        assert_eq!(41, output_1);
    }
}

fn move_guard(map: &mut Map, pos: (usize, usize)) {
    let ipos = (pos.0 as isize, pos.1 as isize);

    match map.get_pos(ipos) {
        Some(Tile::Guard(dir)) => {
            let offset = dir.to_offset();
            let targeted_position = (pos.0 as isize + offset.0, pos.1 as isize + offset.1);
            if let Some(tile) = map.get_pos(targeted_position) {
                match tile {
                    Tile::Empty(_) => {
                        map.set_pos(targeted_position, Tile::Guard(dir.clone()));
                        map.set_pos(ipos, Tile::Empty(true));
                    }
                    Tile::Obstruction => {
                        map.set_pos(ipos, Tile::Guard(rotate_dir(dir.clone())));
                    }
                    _ => unreachable!("{:?}", tile),
                };
            } else {
                map.set_pos(ipos, Tile::Empty(true));
            }
        }
        _ => unreachable!(),
    }
}

fn rotate_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
