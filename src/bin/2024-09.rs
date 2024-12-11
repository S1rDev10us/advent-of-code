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
        .map(|(i, file_id)| {
            let count = file_id.to_digit(10).unwrap() as usize;
            (
                count,
                if i % 2 == 0 {
                    DiskBlock::File(i / 2)
                } else {
                    DiskBlock::Free
                },
                false,
            )
        })
        .collect::<Vec<_>>();

    // dbg!(&disk_map);
    let string_disk_block = &disk_map
        .iter()
        .map(|block| format!("{}", block.1).repeat(block.0))
        .collect::<Vec<_>>()
        .join("");
    if !actual_input {
        assert_eq!(
            "00...111...2...333.44.5555.6666.777.888899",
            string_disk_block // dbg!(string_disk_block)
        );
    }

    while disk_map
        .iter()
        .any(|block| matches!(block, (1.., DiskBlock::Free, _)))
        && !disk_map.is_empty()
    {
        // Remove empty blocks.
        disk_map.retain(|block| matches!(block, (1.., _, _)));

        // Combine adjacent empty blocks
        let mut skip = false;
        let mut offset = 0;
        for (i, _) in disk_map.clone().iter().enumerate() {
            let i = i - offset;
            if i >= disk_map.len() - 1 {
                continue;
            }
            if skip {
                skip = false;
                continue;
            }
            // If two adjacent blocks are free
            if disk_map[i].1 == DiskBlock::Free && disk_map[i + 1].1 == DiskBlock::Free {
                // Merge the free blocks
                disk_map[i].0 += disk_map[i + 1].0;

                disk_map.remove(i + 1);
                skip = true;
                offset += 1;
            }
        }

        // Remove empty blocks at the end
        while disk_map[disk_map.len() - 1].1 == DiskBlock::Free {
            disk_map.pop();
        }

        if !actual_input {
            println!(
                "{}",
                &disk_map
                    .iter()
                    .map(|block| "[".to_owned() + &format!("{}", block.1).repeat(block.0) + "]")
                    .collect::<Vec<_>>()
                    .join("")
            );
        }

        // Get rightmost file block
        let last_block_pos = disk_map.iter().rposition(|block| {
            matches!(block, (1.., DiskBlock::File(_), _)) && !(star_2 && block.2)
        });
        if last_block_pos.is_none() {
            break;
        }
        let last_block_pos = last_block_pos.unwrap();

        disk_map[last_block_pos].2 = true;
        let last_block = disk_map[last_block_pos];
        let required_space = last_block.0;

        // Get leftmost free block
        let free_pos = disk_map.iter().position(|&block| {
            matches!(block, (_, DiskBlock::Free, _)) && (!star_2 || block.0 >= required_space)
        });
        if free_pos.is_none() {
            continue;
        }
        let free_pos = free_pos.unwrap();

        let free_space = disk_map[free_pos].0;

        if free_pos > last_block_pos {
            continue;
        }

        if !actual_input {
            println!("{},{}", required_space, free_space);
        }

        if required_space > free_space {
            // Overwrite empty space and decrease file block space
            disk_map[free_pos].1 = last_block.1;
            disk_map[free_pos].2 = true;
            disk_map.get_mut(last_block_pos).unwrap().0 -= free_space;
        } else {
            disk_map[free_pos].0 -= required_space;
            disk_map[last_block_pos].1 = DiskBlock::Free;
            disk_map.insert(free_pos, last_block);
        }
    }
    // dbg!(&disk_map);

    let output = disk_map
        .iter()
        .flat_map(|block| vec![block.1; block.0])
        .enumerate()
        .map(|(i, block)| {
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
        Some(2858),          // star 2 test
        Some(6272188244509), // star 2 actual
    )
}
