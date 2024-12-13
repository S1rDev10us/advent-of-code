use advent_of_code::prelude::*;
use rayon::prelude::*;

type Num = u64;

fn main() {
    let (input, star_2, actual_input) = initialize_macro!("2024/11", "125 17");

    let mut stones = input
        .split(" ")
        .map(|stone| stone.trim().parse::<Num>().unwrap())
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
    }

    let output = stones.len();
    check_result(
        output,
        star_2,
        actual_input,
        Some(55312),  // star 1 test
        Some(203457), // star 1 actual
        None,         // star 2 test
        None,         // star 2 actual
    )
}
fn process_stones(stones: &[Num]) -> Vec<Num> {
    // dbg!(stones.len());
    let mut proposed_nums = vec![];

    for &stone in stones.iter() {
        if stone == 0 {
            proposed_nums.push(1);
            continue;
        }

        if stone.checked_ilog10().unwrap_or(1) % 2 == 1 {
            let stone_str = stone.to_string();
            proposed_nums.push(stone_str[0..stone_str.len() / 2].parse::<Num>().unwrap());
            proposed_nums.push(stone_str[stone_str.len() / 2..].parse::<Num>().unwrap());
            continue;
        }
        proposed_nums.push(stone * 2024);
    }
    dbg!(stones.len());

    proposed_nums
}
