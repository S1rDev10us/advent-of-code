use anyhow::Result;
fn sort<T>(mut list: Vec<T>) -> Vec<T>
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
    let sorted_lines_a = sort(lines_a.collect::<Vec<i32>>()).into_iter();

    let mut sorted_lines_b = sort(lines_b.collect::<Vec<i32>>()).into_iter();

    let differences = sorted_lines_a.map(|val_a| {
        let val_b = &sorted_lines_b.next().unwrap();
        (val_a - *val_b).abs()
    });

    let val = differences.sum::<i32>();
    println!("{val}");

    Ok(())
}
