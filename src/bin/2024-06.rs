use advent_of_code::initialize_macro;
use std::collections::HashSet;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Guard(Direction),
    Empty(bool),
    Obstruction,
}

#[derive(Debug, Clone)]
struct Map(Vec<Vec<Tile>>);

impl Map {
    fn pos_in_map(&self, pos: (isize, isize)) -> bool {
        pos.0 >= 0
            && pos.1 >= 0
            && self.0.len() > pos.1 as usize
            && self.0[pos.1 as usize].len() > pos.0 as usize
    }
    fn get_pos(&self, pos: (isize, isize)) -> Option<Tile> {
        if !self.pos_in_map(pos) {
            return None;
        }
        Some(self.0[pos.1 as usize][pos.0 as usize].clone())
    }
    fn set_pos(&mut self, pos: (isize, isize), tile: Tile) -> bool {
        if !self.pos_in_map(pos) {
            return false;
        }

        self.0[pos.1 as usize][pos.0 as usize] = tile;
        true
    }
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{:}",
            self.0
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
        )
    }
}

fn main() {
    let (input, is_star_2, is_actual_input) = initialize_macro!(
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

    let base_evalled_map = eval_map(map.clone(), is_actual_input).0;

    let walked_positions = base_evalled_map
        .0
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tile)| (matches!(tile, Tile::Empty(true)), (x, y)))
                .filter(|(walked, _)| *walked)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let output_1 = walked_positions.len();
    dbg!(output_1);
    if is_actual_input {
        assert_eq!(4433, output_1);
    } else {
        assert_eq!(41, output_1);
    }

    if is_star_2 {
        let looping_maps = walked_positions
            .iter()
            .map(|(_, pos)| {
                let mut map_with_obstruction = map.clone();
                map_with_obstruction.set_pos((pos.0 as isize, pos.1 as isize), Tile::Obstruction);
                eval_map(map_with_obstruction, is_actual_input)
            })
            .filter(|(_, did_loop)| *did_loop);

        let output_2 = looping_maps.count();
        dbg!(output_2);
        if is_actual_input {
            assert_eq!(1516, output_2);
        } else {
            assert_eq!(6, output_2);
        }
    }
}

fn eval_map(mut map: Map, is_actual_input: bool) -> (Map, bool) {
    let mut guard_pos: Option<(usize, usize)> = 'guard_pos: {
        for (y, row) in map.0.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if matches!(tile, Tile::Guard(_)) {
                    break 'guard_pos Some((x, y));
                }
            }
        }
        None
    };

    let mut travelled_positions = HashSet::new();

    while guard_pos.is_some() {
        let pos = guard_pos.unwrap();

        let guard = map.get_pos((pos.0 as isize, pos.1 as isize)).unwrap();

        let dir = if let Tile::Guard(dir) = guard {
            dir
        } else {
            unreachable!()
        };

        let pos_with_dir = (guard_pos, dir);

        if travelled_positions.contains(&pos_with_dir) {
            return (map, true);
        }

        travelled_positions.insert(pos_with_dir);

        guard_pos = move_guard(&mut map, pos);

        if !is_actual_input {
            println!("{:}", map);
        }
    }
    (map, false)
}

fn move_guard(map: &mut Map, pos: (usize, usize)) -> Option<(usize, usize)> {
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
                        return Some((targeted_position.0 as usize, targeted_position.1 as usize));
                    }
                    Tile::Obstruction => {
                        map.set_pos(ipos, Tile::Guard(rotate_dir(dir.clone())));
                    }
                    _ => unreachable!("{:?}", tile),
                };
            } else {
                map.set_pos(ipos, Tile::Empty(true));
                return None;
            }
        }
        _ => unreachable!(),
    }
    Some(pos)
}

fn rotate_dir(dir: Direction) -> Direction {
    match dir {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
    }
}
