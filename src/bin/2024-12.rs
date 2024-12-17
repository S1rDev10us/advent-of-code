use advent_of_code::prelude::*;

#[derive(Default, Debug)]
struct Region {
    plant_type: char,
    area: u32,
    perimeter: u32,
}

fn main() {
    let (input, star_2, actual_input) = initialize_macro!(
        "2024/12",
        "\
            RRRRIICCFF\n\
            RRRRIICCCF\n\
            VVRRRCCFFF\n\
            VVRCCCJFFF\n\
            VVVVCJJCFE\n\
            VVIVCCJJEE\n\
            VVIIICJJEE\n\
            MIIIIIJJEE\n\
            MIIISIJEEE\n\
            MMMISSJEEE\n\
        "
    );
    let grid = to_grid_chars(input);

    let mut connected_check_mask: Grid<_> = grid
        .iter_2d()
        .map(|row| row.iter().map(|_| false))
        .collect();

    let mut regions = vec![];

    for (&plant_id, pos) in grid.iter() {
        // only check positions once
        if *connected_check_mask.get_pos(pos).unwrap() {
            continue;
        }
        let mut current_region = Region {
            plant_type: plant_id,
            ..Region::default()
        };

        let mut adjacent_positions = vec![pos];

        while let Some(check_pos) = adjacent_positions.pop() {
            // Position already used
            if *connected_check_mask.get_pos(check_pos).unwrap() {
                continue;
            }
            // println!("{},{}", plant_id, grid.get_pos(check_pos).unwrap());
            // println!("{:?}", adjacent_positions.len());
            // println!("{}", &connected_check_mask);

            connected_check_mask.set_pos(check_pos, true);
            current_region.area += 1;
            current_region.perimeter += 4;

            for (&adjacent_id, adjacent_pos) in grid.adjacent(check_pos) {
                if adjacent_id != plant_id {
                    continue;
                }
                // println!(
                //     "{},{},{}",
                //     plant_id,
                //     grid.get_pos(adjacent_pos).unwrap(),
                //     adjacent_id
                // );
                // println!("{:?}, {:?}", adjacent_pos, check_pos);
                current_region.perimeter -= 1;

                adjacent_positions.push(adjacent_pos);
            }
        }
        // println!("{:?}", &current_region);
        regions.push(current_region);
    }
    // println!("{:?}", &regions);
    // println!("{}", grid);

    let output = regions
        .iter()
        .map(|region| region.area * region.perimeter)
        .sum();
    check_result(
        output,
        star_2,
        actual_input,
        Some(1930),    // star 1 test
        Some(1363682), // star 1 actual
        None,          // star 2 test
        None,          // star 2 actual
    )
}
