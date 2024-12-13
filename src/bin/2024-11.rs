use advent_of_code::prelude::*;

fn main() {
    let (input, star_2, actual_input) = initialize_macro!(
        "2024/11",
        "\
            \n\
        "
    );

    let output;
    check_result(
        output,
        star_2,
        actual_input,
        None, // star 1 test
        None, // star 1 actual
        None, // star 2 test
        None, // star 2 actual
    )
}
