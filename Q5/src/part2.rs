/// For each of the incorrectly-ordered updates,
/// use the page ordering rules to put the page numbers in the right order.
/// For the above example, here are the three incorrectly-ordered updates and their correct orderings:
///
///     75,97,47,61,53 becomes 97,75,47,61,53.
///     61,13,29 becomes 61,29,13.
///     97,13,75,29,47 becomes 97,75,47,29,13.
///
/// After taking only the incorrectly-ordered updates and ordering them correctly,
/// their middle page numbers are 47, 29, and 47. Adding these together produces 123.
use std::collections::BTreeMap;
use std::collections::BTreeSet;

const CONTENTS: &str = include_str!("../metadata/input.txt");

type RulesMap = BTreeMap<u32, BTreeSet<u32>>;

pub fn part2() {
    let mut sections = CONTENTS.split("\n\n");
    let rules_section = sections.next().unwrap().lines().map(|line| line.split('|'));
    let print_section = sections.next().unwrap().lines().map(|line| line.split(','));

    let mut rules_map: RulesMap = BTreeMap::new();
    for mut rule in rules_section {
        let key: u32 = rule.next().unwrap().parse().unwrap();
        let val: u32 = rule.next().unwrap().parse().unwrap();
        rules_map.entry(key).or_default().insert(val);
    }

    let print_line_list: Vec<Vec<u32>> = print_section
        .map(|print_line| {
            print_line
                .map(|page_to_print| page_to_print.parse().unwrap())
                .collect()
        })
        .collect();

    let mut result = 0;
    for mut print_line in print_line_list {
        let (order_status, incorrect_order_idx, swap_page_idx) =
            is_order_correct(&print_line, &rules_map);
        if !order_status {
            swap_till_correct(
                incorrect_order_idx,
                swap_page_idx,
                &rules_map,
                &mut print_line,
            );
            if let Some(mid_elem) = print_line.get(print_line.len() / 2) {
                result += mid_elem;
            }
        }
    }

    println!("result: {result}");
}

fn swap_till_correct(
    mut major_page_idx: usize,
    mut minor_page_idx: usize,
    rules_map: &RulesMap,
    print_line: &mut [u32],
) {
    loop {
        print_line.swap(major_page_idx, minor_page_idx);
        let (order_status, incorrect_order_idx, swap_page_idx) =
            is_order_correct(&print_line, &rules_map);

        if order_status {
            break;
        }
        major_page_idx = incorrect_order_idx;
        minor_page_idx = swap_page_idx;
    }
}

fn is_order_correct(print_line: &[u32], rules_map: &RulesMap) -> (bool, usize, usize) {
    let pos_page: BTreeMap<u32, usize> = print_line
        .iter()
        .enumerate()
        .map(|(index, &page)| (page, index))
        .collect();

    for (key, rule_set) in rules_map {
        if let Some(rule_page_index) = pos_page.get(key) {
            for rule in rule_set {
                if let Some(small_page) = pos_page.get(rule) {
                    if rule_page_index >= small_page {
                        return (false, *rule_page_index, *small_page);
                    }
                }
            }
        }
    }

    (true, 0, 0)
}
