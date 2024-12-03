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

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let pairs = re.captures_iter(input);

    let output_1 = pairs
        .map(|m| (m[1].to_string(), m[2].to_string()))
        .map(|(a, b)| (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap()))
        .map(|(a, b)| a * b)
        .sum::<i32>();

    dbg!(output_1);
    assert_eq!(161289189i32, output_1);
}
