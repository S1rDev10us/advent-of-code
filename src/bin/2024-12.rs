use advent_of_code::prelude::*;
use svg::node::element::path::Data;
use svg::node::element::Text;
use svg::node::element::{Circle, Ellipse, Path, TextPath};
use svg::Document;

#[derive(Default, Debug)]
struct Region {
    plant_type: char,
    area: usize,
    perimeter: usize,
    edges: Vec<(Position<usize>, Direction, usize)>,
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
        // let mut current_region = Region::default();
        let mut current_region = Region {
            plant_type: plant_id,
            ..Region::default()
        };

        let mut adjacent_positions = vec![Position(pos.0, pos.1)];

        while let Some(check_pos) = adjacent_positions.pop() {
            // Position already used
            if *connected_check_mask.get_pos(check_pos).unwrap() {
                continue;
            }

            connected_check_mask.set_pos(check_pos, true);
            current_region.area += 1;
            current_region.perimeter += 4;

            for adjacent_dir in Direction::all_dirs() {
                let adjacent_offset: Offset<_> = adjacent_dir.into();
                let adjacent_pos = check_pos.into_type::<isize>().unwrap()
                    + adjacent_offset.into_type::<isize>().unwrap();
                let adjacent_id = grid.get_signed_pos(adjacent_pos);
                if !grid.contains_signed_point(adjacent_pos) || *adjacent_id.unwrap() != plant_id {
                    current_region
                        .edges
                        .push((check_pos, adjacent_dir.rotate_right(), 1));
                    continue;
                }

                current_region.perimeter -= 1;
                adjacent_positions.push(adjacent_pos.into_type().unwrap());
            }
            current_region.edges = combine_edges(current_region.edges);
        }
        current_region.edges = combine_edges(current_region.edges);
        regions.push(current_region);
    }
    // dbg!(regions
    // .iter()
    // .map(|region| (
    //     region.plant_type,
    //     region.area,
    //     region.edges.len(),
    //     &region.edges
    // ))
    // .collect::<Vec<_>>());

    if !actual_input {
        output_as_svg(&regions, grid);
    }

    let output = regions
        .iter()
        .map(|region| {
            region.area
                * if !star_2 {
                    region.perimeter
                } else {
                    region.edges.len()
                }
        })
        .sum();
    check_result(
        output,
        star_2,
        actual_input,
        Some(1930),    // star 1 test
        Some(1363682), // star 1 actual
        Some(1206),    // star 2 test
        Some(787680),  // star 2 actual
    )
}

fn combine_edges(
    edges: Vec<(Position<usize>, Direction, usize)>,
) -> Vec<(Position<usize>, Direction, usize)> {
    let mut proposed_edges = edges.iter().map(|edge| Some(*edge)).collect::<Vec<_>>();

    for i in 0..proposed_edges.len() {
        let edge1 = proposed_edges[i];
        if edge1.is_none() {
            continue;
        }
        let mut edge1 = edge1.unwrap();

        for (j, edge2) in edges.iter().enumerate() {
            if i == j {
                continue;
            }
            if proposed_edges[j].is_none() {
                continue;
            }
            // Same direction
            if edge1.1 != edge2.1 {
                continue;
            }
            let direction_offset: Offset<_> = edge1.1.into();
            if edge1.0.into_type().unwrap()
                + (direction_offset.into_type::<isize>().unwrap() * edge1.2.try_into().unwrap())
                != edge2.0.into_type().unwrap()
            {
                continue;
            }
            // dbg!("overlap", edge1, edge2);
            edge1.2 += edge2.2;
            proposed_edges[i] = Some(edge1);
            proposed_edges[j] = None;
        }
    }

    // let current_len = edges.len();
    let final_edges: Vec<_> = proposed_edges.into_iter().flatten().collect();
    // dbg!(current_len == final_edges.len());
    final_edges
}
fn output_as_svg(regions: &[Region], grid: Grid<char>) {
    let width = grid.width() * 50 + 45 * 2;
    let height = grid.height() * 50 + 45 * 2;
    let mut document = Document::new().set("viewBox", (0, 0, width, height));
    for region in regions.iter() {
        for (i, edge) in region.edges.iter().enumerate() {
            let mut data = Data::new();
            let into: Offset<_> = edge.1.into();
            let into: Offset<_> = into.into_type::<isize>().unwrap();
            let offset: Offset<_> = into * (edge.2 as isize * 50 - 10);
            let corner = match edge.1 {
                Direction::Up => Offset(-20, 20),
                Direction::Down => Offset(20, -20),
                Direction::Left => Offset(20, 20),
                Direction::Right => Offset(-20, -20),
            };
            let pos: Position<isize> = Position(edge.0 .0 * 50, edge.0 .1 * 50)
                .into_type()
                .unwrap()
                + Offset(70, 70)
                + corner;
            // + (Offset(-into.1, -into.0) * 12);
            data = data.move_to((pos.0, pos.1)).line_by((offset.0, offset.1));
            let color = match edge.1 {
                Direction::Up => "black",
                Direction::Down => "red",
                Direction::Left => "green",
                Direction::Right => "blue",
            };
            let label_text = region.plant_type.to_string() + "-" + &i.to_string();
            let label_text = label_text.as_str();
            let path = Path::new()
                .set("id", label_text)
                .set("fill", "none")
                .set("stroke", color)
                .set("stroke-width", 3)
                .set("d", data.close());
            let start_point = Circle::new()
                .set("fill", color)
                .set("cx", pos.0)
                .set("cy", pos.1)
                .set("r", 5);
            let end_point = Ellipse::new()
                .set("fill", color)
                .set("cx", pos.0 + offset.0)
                .set("cy", pos.1 + offset.1)
                .set(if edge.1.is_vertical() { "ry" } else { "rx" }, 2)
                .set(if edge.1.is_vertical() { "rx" } else { "ry" }, 8);
            let label = TextPath::new(label_text)
                .set("href", "#".to_string() + label_text)
                .set("style", "z-index: 500;");
            document = document.add(path).add(start_point).add(end_point).add(
                Text::new("")
                    .add(label)
                    .set("font-size", "0.5em")
                    .set("style", "z-index: 500;")
                    .set("fill", "grey"),
            );
        }
    }
    for i in 0..grid.width() + 1 {
        document = document.add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "grey")
                .set("stroke-width", 1)
                .set(
                    "d",
                    Data::new()
                        .move_to((i * 50 + 45, 45))
                        .line_by((0, grid.height() * 50))
                        .close(),
                ),
        );
    }
    for i in 0..grid.height() + 1 {
        document = document.add(
            Path::new()
                .set("fill", "none")
                .set("stroke", "grey")
                .set("stroke-width", 1)
                .set(
                    "d",
                    Data::new()
                        .move_to((45, i * 50 + 45))
                        .line_by((grid.width() * 50, 0))
                        .close(),
                ),
        );
    }

    svg::save("image.svg", &document).unwrap();
}
