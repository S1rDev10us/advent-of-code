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

    let output;
    check_result(
        output,
        star_2,
        actual_input,
        Some(36), // star 1 test
        None,     // star 1 actual
        None,     // star 2 test
        None,     // star 2 actual
    )
}
