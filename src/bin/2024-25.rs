use advent_of_code::prelude::*;

fn main() {
    let (input, star_2, actual_input) = initialize_macro!(
        "2024/25",
        "\
            #####\n\
            .####\n\
            .####\n\
            .####\n\
            .#.#.\n\
            .#...\n\
            .....\n\
            \n\
            #####\n\
            ##.##\n\
            .#.##\n\
            ...##\n\
            ...#.\n\
            ...#.\n\
            .....\n\
            \n\
            .....\n\
            #....\n\
            #....\n\
            #...#\n\
            #.#.#\n\
            #.###\n\
            #####\n\
            \n\
            .....\n\
            .....\n\
            #.#..\n\
            ###..\n\
            ###.#\n\
            ###.#\n\
            #####\n\
            \n\
            .....\n\
            .....\n\
            .....\n\
            #....\n\
            #.#..\n\
            #.#.#\n\
            #####\
        "
    );
    let patterns = input
        .trim()
        .split("\n\n")
        .map(|pattern| pattern.trim().split("\n").collect::<Vec<_>>())
        .map(|pattern| {
            let is_key = !pattern[0].chars().all(|val| val == '#');
            (is_key, pattern)
        })
        .map(|(is_key, pattern)| {
            (
                is_key,
                if !is_key {
                    pattern.into_iter().rev().collect::<Vec<&str>>()
                } else {
                    pattern
                },
            )
        })
        .map(|(is_key, pattern)| {
            let columns = pattern.iter().fold(
                pattern[0].chars().map(|_| -1).collect::<Vec<_>>(),
                |acc, row| {
                    let mut acc = acc;
                    for (i, tile) in row.chars().enumerate() {
                        if tile != '#' {
                            continue;
                        }
                        acc[i] += 1;
                    }
                    acc
                },
            );
            (is_key, pattern, columns)
        })
        .collect::<Vec<_>>();

    let locks = patterns
        .iter()
        .filter(|(is_key, ..)| !is_key)
        .collect::<Vec<_>>();
    let keys = patterns
        .iter()
        .filter(|(is_key, ..)| *is_key)
        .collect::<Vec<_>>();

    dbg!(&patterns, &locks, &keys);

    let mut combos = 0;

    for lock in &locks {
        'key: for key in &keys {
            for i in 0..lock.2.len() {
                let used_space = key.2[i] + lock.2[i];
                if used_space > lock.1.len() as i16 - 2 {
                    continue 'key;
                }
            }
            combos += 1;
        }
    }

    let output = combos;
    check_result(
        output,
        star_2,
        actual_input,
        Some(3),    // star 1 test
        Some(2691), // star 1 actual
        None,       // star 2 test
        None,       // star 2 actual
    )
}
