use advent_of_code::prelude::*;
use std::fmt;
use std::fmt::Display;

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
        .iter_2d()
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

            let signed_ant_1: Position<_> =
                (antenna_1.1 .0 as isize, antenna_1.1 .1 as isize).into();
            let signed_ant_2: Position<_> =
                (antenna_2.1 .0 as isize, antenna_2.1 .1 as isize).into();

            let offset: Offset<_> = signed_ant_1 - signed_ant_2;
            if !star_2 {
                let pos_1 = offset + signed_ant_1;

                if grid.contains_signed_point(pos_1) {
                    antenna_mask.set_signed_pos(pos_1, true);
                }

                let pos_2 = signed_ant_2 - offset;

                if grid.contains_signed_point(pos_2) {
                    antenna_mask.set_signed_pos(pos_2, true);
                }
            } else {
                for i in 0.. {
                    let pos = signed_ant_1 + (offset * i);
                    if grid.contains_signed_point(pos) {
                        antenna_mask.set_signed_pos(pos, true);
                    } else {
                        break;
                    }
                }

                for i in 0.. {
                    let pos = signed_ant_2 - (offset * i);
                    if grid.contains_signed_point(pos) {
                        antenna_mask.set_signed_pos(pos, true);
                    } else {
                        break;
                    }
                }
            }
        }
    }
    println!("{}", &grid);
    println!(
        "{}",
        format!("{}", &antenna_mask)
            .replace("false", ".")
            .replace("true", "#")
    );

    let output = antenna_mask
        .iter()
        .filter(|is_antenna| *is_antenna.0)
        .count();
    dbg!(&output);

    assert_eq!(
        match (star_2, is_actual_input) {
            (false, true) => 249,
            (false, false) => 14,

            (true, true) => 905,
            (true, false) => 34,
        },
        output
    );
}
