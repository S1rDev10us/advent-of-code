use advent_of_code::initialize_macro;

fn main() {
    let (input, _, is_actual_input) = initialize_macro!(
        "2024/07",
        "\
        190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20\
        "
    );

    let equations = input
        .split("\n")
        .map(|equation| equation.trim())
        .filter(|equation| equation != &"")
        .map(|equation| {
            let mut parts = equation.split(':');
            let target = parts.next();
            // dbg!(equation);
            let values = parts.next();
            (
                target.unwrap().parse::<u64>().unwrap(),
                values
                    .unwrap()
                    .trim()
                    .split(" ")
                    .map(|val| val.parse::<u64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<(u64, Vec<u64>)>>();

    // dbg!(&equations);

    let output_1 = equations
        .iter()
        .map(|(target, values)| (target, check_operations(target, 0, values)))
        .filter(|(_, reached_target)| *reached_target)
        .map(|(target, _)| target)
        .sum::<u64>();
    dbg!(output_1);
    if is_actual_input {
        assert_eq!(6083020304036, output_1);
    } else {
        assert_eq!(3749, output_1);
    }
}

fn check_operations(target: &u64, acc: u64, values: &[u64]) -> bool {
    let operations = [
        |a: u64, b: u64| -> u64 { a + b },
        |a: u64, b: u64| -> u64 {
            if a == 0 {
                b
            } else {
                a * b
            }
        },
    ];
    if &acc > target {
        return false;
    }
    if values.is_empty() {
        return &acc == target;
    }

    operations
        .iter()
        .any(|op| check_operations(target, op(acc, values[0]), &values[1..]))
}
