use advent_of_code::prelude::*;

type Num = u64;

fn main() {
    let (input, star_2, actual_input) = initialize_macro!("2024/11", "125 17");

    let mut stones = input
        .split(" ")
        .map(|stone| stone.trim().parse::<Num>().unwrap())
        .collect::<Vec<_>>();

    let iterations = 25;

    for _ in 0..iterations {
        let mut proposed_nums = vec![];

        for &stone in stones.iter() {
            if stone == 0 {
                proposed_nums.push(1);
                continue;
            }

            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                proposed_nums.push(stone_str[0..stone_str.len() / 2].parse::<Num>().unwrap());
                proposed_nums.push(stone_str[stone_str.len() / 2..].parse::<Num>().unwrap());
                continue;
            }
            proposed_nums.push(stone * 2024);
        }

        stones = proposed_nums;
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
