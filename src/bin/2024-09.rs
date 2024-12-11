use advent_of_code::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DiskBlock {
    File(usize),
    Free,
}

display!(DiskBlock, self, {
    match self {
        DiskBlock::File(id) => id.to_string(),
        DiskBlock::Free => '.'.into(),
    }
});

fn main() {
    let (input, star_2, actual_input) = initialize_macro!("2024/09", "2333133121414131402");

    let mut disk_map = input
        .trim()
        .chars()
        .enumerate()
        .flat_map(|(i, file_id)| {
            let count = file_id.to_digit(10).unwrap();
            vec![
                if i % 2 == 0 {
                    DiskBlock::File(i / 2)
                } else {
                    DiskBlock::Free
                };
                count as usize
            ]
        })
        .collect::<Vec<_>>();

    // dbg!(&disk_map);
    let string_disk_block = &disk_map
        .iter()
        .map(|block| format!("{}", block))
        .collect::<Vec<_>>()
        .join("");
    if !actual_input {
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            string_disk_block // dbg!(string_disk_block)
        );
    }

    while disk_map.contains(&DiskBlock::Free) && !disk_map.is_empty() {
        let last_block = disk_map.pop().unwrap();
        if matches!(last_block, DiskBlock::Free) {
            continue;
        }

        let free_pos = disk_map
            .iter()
            .position(|&block| block == DiskBlock::Free)
            .unwrap();

        disk_map[free_pos] = last_block;
    }
    // dbg!(&disk_map);

    let output = disk_map
        .iter()
        .enumerate()
        .map(|(i, &block)| {
            if let DiskBlock::File(id) = block {
                i * id
            } else {
                0
            }
        })
        .sum::<usize>();

    check_result(
        output,
        star_2,
        actual_input,
        Some(1928),          // star 1 test
        Some(6242766523059), // star 1 actual
        None,                // star 2 test
        None,                // star 2 actual
    )
}
