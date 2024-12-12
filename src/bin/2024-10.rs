use advent_of_code::prelude::*;

fn main() {
    let (input, star_2, actual_input) = initialize_macro!(
        "2024/10",
        "\
            89010123\n\
            78121874\n\
            87430965\n\
            96549874\n\
            45678903\n\
            32019012\n\
            01329801\n\
            10456732\
        "
    );
    let grid = to_grid(input, |tile| tile.to_string().parse::<u8>().unwrap());

    let scores = grid
        .iter()
        .filter(|(&tile, _)| tile == 0)
        .map(|(_, (x, y))| {
            let mut expanded_area = true;
            let mut trail_mask = grid
                .iter_2d()
                .map(|row| row.iter().map(|_| false))
                .collect::<Grid<_>>();
            trail_mask.set_pos((x, y), true);

            // Keep expanding area until exhausted search space
            while expanded_area {
                expanded_area = false;

                // Potential optimization: only search next to the most recently found positions

                // The heights and positions of all tiles connected to the trailhead
                let connected_tiles = trail_mask
                    .iter()
                    .filter(|&(&tile, _)| tile)
                    .map(|(_, pos)| (grid.get_pos(pos).unwrap(), pos))
                    .collect::<Vec<_>>();

                for (height, pos) in connected_tiles {
                    let adjacent_tiles = [(0, -1), (0, 1), (-1, 0), (1, 0)]
                        .iter()
                        .map(|offset| (offset.0 + pos.0 as isize, offset.1 + pos.1 as isize))
                        .filter(|pos2| grid.contains_signed_point(*pos2))
                        .map(|pos| (pos.0 as usize, pos.1 as usize))
                        .map(|pos| (grid.get_pos(pos).unwrap(), pos));
                    for (height2, pos2) in adjacent_tiles {
                        if *height2 != height + 1 {
                            continue;
                        }
                        if *trail_mask.get_pos(pos2).unwrap() {
                            continue;
                        }

                        expanded_area = true;
                        trail_mask.set_pos(pos2, true);
                    }
                }
            }
            trail_mask
                .iter()
                .filter(|&(&tile, _)| tile)
                .filter(|&(_, pos)| matches!(grid.get_pos(pos), Some(9)))
                .count()
        })
        .collect::<Vec<_>>();

    let output = scores.iter().sum();
    check_result(
        output,
        star_2,
        actual_input,
        Some(36),  // star 1 test
        Some(737), // star 1 actual
        None,      // star 2 test
        None,      // star 2 actual
    )
}
