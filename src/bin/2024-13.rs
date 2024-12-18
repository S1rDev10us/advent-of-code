use advent_of_code::prelude::*;

fn main() {
    let (input, star_2, actual_input) = initialize_macro!(
        "2024/13",
        "\
            Button A: X+94, Y+34\n\
            Button B: X+22, Y+67\n\
            Prize: X=8400, Y=5400\n\
            \n\
            Button A: X+26, Y+66\n\
            Button B: X+67, Y+21\n\
            Prize: X=12748, Y=12176\n\
            \n\
            Button A: X+17, Y+86\n\
            Button B: X+84, Y+37\n\
            Prize: X=7870, Y=6450\n\
            \n\
            Button A: X+69, Y+23\n\
            Button B: X+27, Y+71\n\
            Prize: X=18641, Y=10279\n\
        "
    );
    let machines = input
        .split("\n\n")
        .map(|machine| {
            let machine_lines = machine.trim().split("\n").collect::<Vec<_>>();
            let no_lines = machine_lines.len();
            let prize = machine_lines[no_lines - 1];
            (
                (Position::from(to_coords(&prize[7..], true))),
                machine_lines[0..no_lines - 1]
                    .to_vec()
                    .iter()
                    .map(|&button| {
                        (
                            button.chars().nth(7).unwrap(),
                            Offset::from(to_coords(&button[10..], false)),
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();

    let max_iterations = 100;

    let optimal_tokens = machines
        .iter()
        .flat_map(|(target, buttons)| {
            dbg!(target, buttons);
            let mut min_token_cost = usize::MAX;
            const BUTTON_A_COST: usize = 3;
            const BUTTON_B_COST: usize = 1;
            let mut buttons_used = (0, 0);
            for buttons_a in 0..max_iterations {
                let button_a_cost = buttons_a * BUTTON_A_COST;
                if button_a_cost > min_token_cost {
                    // dbg!("Cost too high {},{}", button_a_cost, min_token_cost);
                    continue;
                }

                let button_a_offset = buttons[0].1 * buttons_a as isize;

                if button_a_offset.0 > target.0 || button_a_offset.1 > target.1 {
                    break;
                }

                for buttons_b_used in 0..max_iterations {
                    let new_pos = Position(0, 0)
                        + (button_a_offset + (buttons[1].1 * buttons_b_used as isize));
                    // println!("{},{}", buttons_a, buttons_b_used);
                    if new_pos.0 > target.0 || new_pos.1 > target.1 {
                        break;
                    }
                    if &new_pos != target {
                        continue;
                    }
                    let total_cost = button_a_cost + (BUTTON_B_COST * buttons_b_used);
                    // dbg!(total_cost);

                    if total_cost < min_token_cost {
                        min_token_cost = total_cost;
                        buttons_used = (buttons_a, buttons_b_used);
                    }
                }
            }
            (min_token_cost != usize::MAX).then_some((min_token_cost, buttons_used))
        })
        .collect::<Vec<_>>();
    dbg!(&machines);
    dbg!(&optimal_tokens);

    let output = optimal_tokens
        .iter()
        .map(|(token_cost, _)| token_cost)
        .sum();
    check_result(
        output,
        star_2,
        actual_input,
        Some(480),   // star 1 test
        Some(37901), // star 1 actual
        None,        // star 2 test
        None,        // star 2 actual
    )
}

fn to_coords(coord_set: &str, skip_sign: bool) -> (isize, isize) {
    let mut coords = coord_set
        .split(", ")
        .map(|coord| &coord[1 + usize::from(skip_sign)..])
        .map(|coord| coord.parse::<isize>().unwrap());
    (coords.next().unwrap(), coords.next().unwrap())
}
