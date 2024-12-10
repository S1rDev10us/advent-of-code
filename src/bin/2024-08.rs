use advent_of_code::{initialize_macro, to_grid, Grid};
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Sub};

enum Tile {
    Antenna(char),
    Empty,
}
impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Antenna(c) => c,
                Tile::Empty => &'.',
            }
        )
    }
}

fn main() {
    let (input, star_2, is_actual_input) = initialize_macro!(
        "2024/08",
        "\
            ............\n\
            ........0...\n\
            .....0......\n\
            .......0....\n\
            ....0.......\n\
            ......A.....\n\
            ............\n\
            ............\n\
            ........A...\n\
            .........A..\n\
            ............\n\
            ............\n\
        "
    );

    let grid = to_grid(input, |tile| match tile {
        '.' => Tile::Empty,
        _ => Tile::Antenna(tile),
    });

    let antennae = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, tile)| (tile, (x, y)))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut antenna_mask = Grid::new(vec![vec![false; grid.width()]; grid.height()]);

    for antenna_1 in antennae.iter() {
        for antenna_2 in antennae.iter() {
            if antenna_1.1 == antenna_2.1 {
                continue;
            }

            if let Tile::Antenna(freq_1) = antenna_1.0 {
                if let Tile::Antenna(freq_2) = antenna_2.0 {
                    if freq_1 != freq_2 {
                        continue;
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            }

            let signed_ant_1 = (antenna_1.1 .0 as isize, antenna_1.1 .1 as isize);
            let signed_ant_2 = (antenna_2.1 .0 as isize, antenna_2.1 .1 as isize);

            let offset = sub::<isize>(signed_ant_1, signed_ant_2);

            let pos_1 = add(signed_ant_1, offset);

            if grid.contains_signed_point(pos_1) {
                antenna_mask.set_signed_pos(pos_1, true);
            }

            let pos_2 = sub(signed_ant_2, offset);
            if grid.contains_signed_point(pos_2) {
                antenna_mask.set_signed_pos(pos_2, true);
            }
        }
    }
    println!("{}", &grid);

    let output_1 = antenna_mask
        .iter()
        .flatten()
        .filter(|is_antenna| **is_antenna)
        .count();
    dbg!(&output_1);
    if is_actual_input {
        // assert_eq!(4433, output_1);
    } else {
        assert_eq!(14, output_1);
    }
}
fn add<T>(a: (T, T), b: (T, T)) -> (T, T)
where
    T: Add<T, Output = T>,
{
    (a.0 + b.0, a.1 + b.1)
}
fn sub<T>(a: (T, T), b: (T, T)) -> (T, T)
where
    T: Sub<T, Output = T>,
{
    (a.0 - b.0, a.1 - b.1)
}
