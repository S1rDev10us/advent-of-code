use advent_of_code::initialize_macro;

fn main() {
    let (input, _is_star_2, is_actual_input) = initialize_macro!(
        "\
        MMMSXXMASM\n\
        MSAMXMSMSA\n\
        AMXSXMAAMM\n\
        MSAMASMSMX\n\
        XMASAMXAMM\n\
        XXAMMXXAMA\n\
        SMSMSASXSS\n\
        SAXAMASAAA\n\
        MAMMMXMMMM\n\
        MXMXAXMASX\
        "
    );
    let lines = input
        .split('\n')
        .filter(|line| line.trim() != "")
        .collect::<Vec<_>>();
    // ["MMMSX...", ...]

    let grid = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<_>>>();

    let dirs = [(0, 1), (1, 0), (1, 1), (1, -1)];
    let mut star_1_matches = 0;

    for dir in dirs {
        for (x, line) in lines.iter().enumerate() {
            for y in 0..line.len() {
                if search_line(&grid, (x, y), dir) {
                    star_1_matches += 1;
                }
            }
        }
    }
    dbg!(star_1_matches);
    if is_actual_input {
        assert_eq!(2685, star_1_matches);
    } else {
        assert_eq!(18, star_1_matches);
    }

    let mut star_2_matches = 0;
    for x in 1..lines.len() - 1 {
        let line = lines[x];
        for y in 1..line.len() - 1 {
            if search_x(&grid, (x as isize, y as isize)) {
                star_2_matches += 1;
            }
        }
    }
    dbg!(star_2_matches);
    if is_actual_input {
        assert_eq!(2048, star_2_matches);
    } else {
        assert_eq!(9, star_2_matches);
    }
}

fn search_x(grid: &[Vec<char>], (x, y): (isize, isize)) -> bool {
    if let Some('A') = get_tile(grid, x, y) {
        let positions = [
            get_tile(grid, x + 1, y + 1),
            get_tile(grid, x - 1, y - 1),
            get_tile(grid, x - 1, y + 1),
            get_tile(grid, x + 1, y - 1),
        ];
        if positions.iter().any(|pos| pos.is_none()) {
            return false;
        }

        if positions.iter().any(|pos| {
            let u_pos = pos.unwrap();
            u_pos != 'M' && u_pos != 'S'
        }) {
            return false;
        }

        // Positions on opposite diagonals are not equivalent
        if positions[0] == positions[1] || positions[2] == positions[3] {
            return false;
        }

        return true;
    }
    false
}

fn search_line(
    grid: &[Vec<char>],
    (x_start, y_start): (usize, usize),
    (x_off, y_off): (isize, isize),
) -> bool {
    let mut forward: Option<bool> = None;
    let search_val = "XMAS";
    let mut successful = true;
    for search_index in 0..search_val.len() {
        if let Some(tile) = get_tile(
            grid,
            x_start as isize + search_index as isize * x_off,
            y_start as isize + search_index as isize * y_off,
        ) {
            match forward {
                Some(is_forward) => {
                    let desired_char = if is_forward {
                        search_val.chars().nth(search_index)
                    } else {
                        search_val.chars().nth(search_val.len() - 1 - search_index)
                    }
                    .unwrap();

                    if desired_char != tile {
                        successful = false;
                        break;
                    }
                }
                None => {
                    if tile == 'X' {
                        forward = Some(true);
                    } else if tile == 'S' {
                        forward = Some(false);
                    } else {
                        successful = false;
                        break;
                    }
                }
            }
        } else {
            successful = false;
            break;
        }
    }
    successful
}

fn get_tile(grid: &[Vec<char>], x: isize, y: isize) -> Option<char> {
    if x < 0 || y < 0 {
        return None;
    }
    let x = x as usize;
    let y = y as usize;

    if grid.len() < y + 1 {
        return None;
    }
    if grid[y].len() < x + 1 {
        return None;
    }
    Some(grid[y][x])
}

// tests

#[test]
fn tile_outside_grid_is_none() {
    let grid = vec![vec![]];

    let tile = get_tile(&grid, 1, 0);
    assert_eq!(tile, None);

    let tile = get_tile(&grid, 0, 1);
    assert_eq!(tile, None);

    let tile = get_tile(&grid, 1, 1);
    assert_eq!(tile, None);
}

#[test]
fn tile_in_grid_is_some() {
    let grid = vec![vec!['a']];

    let tile = get_tile(&grid, 0, 0);
    assert_eq!(tile, Some('a'));
}
