use advent_of_code::initialize_macro;
use std::cmp::Ordering;
use std::collections::HashMap;

fn main() {
    let (input, _, is_actual_input) = initialize_macro!(
        "2024/05",
        "\
            47|53\n\
            97|13\n\
            97|61\n\
            97|47\n\
            75|29\n\
            61|13\n\
            75|53\n\
            29|13\n\
            97|29\n\
            53|29\n\
            61|53\n\
            97|53\n\
            61|29\n\
            47|13\n\
            75|47\n\
            97|75\n\
            47|61\n\
            75|61\n\
            47|29\n\
            75|13\n\
            53|13\n\
            \n\
            75,47,61,53,29\n\
            97,61,53,29,13\n\
            75,29,13\n\
            75,97,47,61,53\n\
            61,13,29\n\
            97,13,75,29,47\
        "
    );
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let rules = sections[0]
        .split("\n")
        .map(|rule| rule.trim().split('|'))
        .map(|mut rule| (rule.next().unwrap(), rule.next().unwrap()))
        // .map(|rule| {
        //     (
        //         rule.0.parse::<i32>().unwrap(),
        //         rule.1.parse::<i32>().unwrap(),
        //     )
        // })
        .fold(HashMap::<_, Vec<_>>::new(), |mut map, rule| {
            if let Some(rule_list) = map.get_mut(&rule.1) {
                rule_list.push(rule.0);
            } else {
                map.insert(rule.1, vec![rule.0]);
            }
            map
        });

    // dbg!(&rules);

    let updates = sections[1]
        .split('\n')
        .filter(|update| update.trim() != "")
        .map(|update| update.split(','))
        .map(|update| update.collect::<Vec<_>>())
        .map(|update| {
            let mut found = vec![];
            let mut blacklist = vec![];
            (
                update.iter().all(|item| {
                    // dbg!(&blacklist, item);
                    if blacklist.contains(item) {
                        return false;
                    }
                    if !found.contains(&item) {
                        if let Some(new_rules) = rules.get(item) {
                            blacklist.append(&mut new_rules.clone());
                        }
                        found.push(item);
                    }
                    true
                }),
                update,
            )
        })
        .collect::<Vec<_>>();

    let output_1 = updates
        .iter()
        .filter(|update| update.0)
        .map(|update| &update.1)
        .map(|update| {
            // dbg!(&update);
            update[update.len() / 2].parse::<i32>().unwrap()
        })
        .sum::<i32>();
    dbg!(output_1);

    if is_actual_input {
        assert_eq!(output_1, 4872);
    } else {
        assert_eq!(output_1, 143);
    }

    let output_2 = updates
        .iter()
        .filter(|update| !update.0)
        .map(|update| update.1.clone())
        .map(|mut update| {
            update.sort_by(|a, b| {
                if a == b {
                    return Ordering::Equal;
                }
                if let Some(rule_list) = rules.get(a) {
                    if rule_list.contains(b) {
                        return Ordering::Less;
                    }
                }
                if let Some(rule_list) = rules.get(b) {
                    if rule_list.contains(a) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });
            update
        })
        .map(|update| {
            // dbg!(&update);
            update[update.len() / 2].parse::<i32>().unwrap()
        })
        .sum::<i32>();
    dbg!(output_2);

    if is_actual_input {
        assert_eq!(output_2, 5564);
    } else {
        assert_eq!(output_2, 123);
    }
}
