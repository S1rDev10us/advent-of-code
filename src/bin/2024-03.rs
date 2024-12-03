use regex::Regex;

fn main() {
    let actual_input = include_str!("../../puzzles/2024/03/input");
    let test_input = "\
        ";
    let using_actual_input = true;

    let input = if using_actual_input {
        actual_input
    } else {
        test_input
    };

    let re = Regex::new(r"do(n't)?\(\)|mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    let pairs = re.captures_iter(input);

    let mut enabled = true;

    let output_2 = pairs
        .filter(|captures| {
            let str_mtch = &captures[0];
            dbg!(str_mtch, enabled);
            if !str_mtch.starts_with("do") {
                enabled
            } else {
                enabled = !str_mtch.starts_with("don't");
                false
            }
        })
        .map(|captures| (captures["a"].to_string(), captures["b"].to_string()))
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .map(|(a, b)| a * b)
        .sum::<i32>();

    dbg!(output_2);
    // assert_eq!(161289189i32, output_1);
    assert_eq!(83595109i32, output_2);
}
