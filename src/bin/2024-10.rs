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
        .map(|(_, pos)| {
            let mut heads = vec![(pos, 1)];
            for height in 0..9 {
                let mut proposed_heads = vec![];
                for (pos, no_heads) in heads.iter() {
                    let adjacent_tiles = [(0, -1), (0, 1), (-1, 0), (1, 0)]
                        .iter()
                        .map(|offset| (offset.0 + pos.0 as isize, offset.1 + pos.1 as isize))
                        .filter(|pos| grid.contains_signed_point(*pos))
                        .map(|pos| (pos.0 as usize, pos.1 as usize))
                        .map(|pos| (grid.get_pos(pos).unwrap(), pos));
                    for (&height2, pos2) in adjacent_tiles {
                        if height2 != height + 1 {
                            continue;
                        }

                        proposed_heads.push((pos2, *no_heads));
                    }
                }
                heads = proposed_heads;

                // Merge identical heads
                heads.sort_by_key(|head| head.0);
                heads.dedup_by(|a, b| {
                    if a.0 != b.0 {
                        return false;
                    }
                    b.1 += a.1;
                    true
                })
            }

            if !star_2 {
                heads.len()
            } else {
                heads.iter().map(|(_, no_heads)| no_heads).sum()
            }
        })
        .collect::<Vec<_>>();

    let output = scores.iter().sum();
    check_result(
        output,
        star_2,
        actual_input,
        Some(36),   // star 1 test
        Some(737),  // star 1 actual
        Some(81),   // star 2 test
        Some(1619), // star 2 actual
    )
}
