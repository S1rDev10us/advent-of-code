use advent_of_code::initialize_macro;
use std::collections::HashMap;

fn main() {
    let (input, _is_star_2, _is_actual_input) = initialize_macro!(
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

    dbg!(&rules);

    let updates = sections[1]
        .split('\n')
        .filter(|update| update.trim() != "")
        .map(|update| update.split(','))
        .map(|update| update.collect::<Vec<_>>())
        .filter(|update| {
            let mut blacklist = vec![];
            update.iter().all(|item| {
                dbg!(&blacklist, item);
                if blacklist.contains(item) {
                    return false;
                }
                if let Some(new_rules) = rules.get(item) {
                    blacklist.append(&mut new_rules.clone());
                }
                true
            })
        });

    let output_1 = updates
        .map(|update| {
            dbg!(&update);
            update[update.len() / 2].parse::<i32>().unwrap()
        })
        .sum::<i32>();
    dbg!(output_1);

    if _is_actual_input {
        assert_eq!(output_1, 4872);
    } else {
        assert_eq!(output_1, 143);
    }
}
