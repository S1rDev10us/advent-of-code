use advent_of_code::prelude::*;
use rayon::prelude::*;
use std::fs;

type Num = u64;
const WORK_PATH: &str = "./workenv/2024/11";

fn main() {
    let (input, star_2, actual_input) = initialize_macro!("2024/11", "125 17");

    let mut stones = input
        .split(" ")
        .map(|stone| (stone.trim().parse::<Num>().unwrap(), 1))
        .collect::<Vec<_>>();

    let iterations = if !star_2 { 25 } else { 75 };

    for i in 0..iterations {
        dbg!(i, stones.len());

        stones = stones
            .chunks(1.max(stones.len() / 16))
            .collect::<Vec<_>>()
            .into_par_iter()
            .flat_map(process_stones)
            .collect();
        merge_dupes(&mut stones);
    }

    let output = stones.iter().map(|stone| stone.1).sum();
    check_result(
        output,
        star_2,
        actual_input,
        Some(55312),           // star 1 test
        Some(203457),          // star 1 actual
        None,                  // star 2 test
        Some(241394363462435), // star 2 actual
    )
}
fn process_stones(stones: &[(Num, usize)]) -> Vec<(Num, usize)> {
    // dbg!(stones.len());
    let mut proposed_nums = vec![];

    for &stone in stones.iter() {
        if stone.0 == 0 {
            proposed_nums.push((1, stone.1));
            continue;
        }

        if stone.0.checked_ilog10().unwrap_or(0) % 2 == 1 {
            let stone_str = stone.0.to_string();
            proposed_nums.push((
                stone_str[0..stone_str.len() / 2].parse::<Num>().unwrap(),
                stone.1,
            ));
            proposed_nums.push((
                stone_str[stone_str.len() / 2..].parse::<Num>().unwrap(),
                stone.1,
            ));
            continue;
        }
        proposed_nums.push((stone.0 * 2024, stone.1));
    }

    proposed_nums
}
