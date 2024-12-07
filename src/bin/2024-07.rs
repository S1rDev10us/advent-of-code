use advent_of_code::initialize_macro;

fn main() {
    let (input, star_2, is_actual_input) = initialize_macro!(
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
    let mut operations = vec![|a: u64, b: u64| -> u64 { a + b }, |a: u64, b: u64| -> u64 {
        if a == 0 {
            b
        } else {
            a * b
        }
    }];
    if star_2 {
        operations.push(|a: u64, b: u64| -> u64 {
            (a.to_string() + &b.to_string()).parse::<u64>().unwrap()
        });
    }

    let output = equations
        .iter()
        .map(|(target, values)| (target, check_operations(target, 0, values, &operations)))
        .filter(|(_, reached_target)| *reached_target)
        .map(|(target, _)| target)
        .sum::<u64>();
    dbg!(output);

    assert_eq!(
        match (star_2, is_actual_input) {
            (false, true) => 6083020304036,
            (false, false) => 3749,

            (true, true) => 59002246504791,
            (true, false) => 11387,
        },
        output
    );
}

fn check_operations(
    target: &u64,
    acc: u64,
    values: &[u64],
    operations: &[fn(u64, u64) -> u64],
) -> bool {
    if &acc > target {
        return false;
    }
    if values.is_empty() {
        return &acc == target;
    }

    operations
        .iter()
        .any(|op| check_operations(target, op(acc, values[0]), &values[1..], operations))
}
