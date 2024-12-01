use anyhow::Result;
use std::collections::HashMap;

fn sort<T>(list: Vec<T>) -> Vec<T>
where
    T: Ord,
{
    let mut lines = list;
    lines.sort();
    lines
}

fn main() -> Result<()> {
    let actual_input = include_str!("../../puzzles/2024/01/input");
    let test_input = "\
        3   4\n\
        4   3\n\
        2   5\n\
        1   3\n\
        3   9\n\
        3   3";
    let input = if true { actual_input } else { test_input };

    let lines = input.split('\n');
    // ["","","3   4", "4   3", "2   5", ...]
    let non_empty_lines = lines.filter(|line| line.trim() != "");
    // ["3   4", "4   3", "2   5", ...]
    let split_lines = non_empty_lines.map(|line| line.split("   "));
    // [["3", "4"], ["4", "3"], ...]
    let lines_a = split_lines
        .clone()
        .map(|mut line| line.next().unwrap().parse::<i32>().unwrap());
    // ["3", "4", ...]
    let lines_b = split_lines
        .clone()
        .map(|mut line| line.nth(1).unwrap().parse::<i32>().unwrap());
    // ["4", "3", ...]

    /////////////////////////////// Star 1

    let sorted_lines_a = sort(lines_a.clone().collect::<Vec<i32>>()).into_iter();

    let mut sorted_lines_b = sort(lines_b.clone().collect::<Vec<i32>>()).into_iter();

    let differences = sorted_lines_a.map(|val_a| {
        let val_b = &sorted_lines_b.next().unwrap();
        (val_a - *val_b).abs()
    });

    let star_1_val = differences.sum::<i32>();
    println!("{star_1_val}"); // 1388114
    assert_eq!(1388114i32, star_1_val);

    /////////////////////////////// Star 2

    let mut similarity_values: HashMap<i32, usize> = HashMap::new();

    let similarities = lines_a.map(|line_a| {
        *(similarity_values.entry(line_a).or_insert(
            line_a as usize * lines_b.clone().filter(|line_b| line_b == &line_a).count(),
        ))
    });
    let star_2_val = similarities.sum::<usize>();
    println!("{star_2_val}"); // 23529853
    assert_eq!(23529853usize, star_2_val);

    Ok(())
}
