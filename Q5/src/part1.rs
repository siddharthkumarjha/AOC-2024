/// The notation X|Y means that if both page number X and page number Y are to be produced as part of an update,
/// page number X must be printed at some point before page number Y.
///
/// The Elf has for you both the page ordering rules and the pages to produce in each update (your puzzle input),
/// but can't figure out whether each update has the pages in the right order.
///
/// For example:
///
/// 47|53
/// 97|13
/// 97|61
/// 97|47
/// 75|29
/// 61|13
/// 75|53
/// 29|13
/// 97|29
/// 53|29
/// 61|53
/// 97|53
/// 61|29
/// 47|13
/// 75|47
/// 97|75
/// 47|61
/// 75|61
/// 47|29
/// 75|13
/// 53|13
///
/// 75,47,61,53,29
/// 97,61,53,29,13
/// 75,29,13
/// 75,97,47,61,53
/// 61,13,29
/// 97,13,75,29,47
///
/// The first section specifies the page ordering rules, one per line.
/// The first rule, 47|53, means that if an update includes both page number 47 and page number 53,
/// then page number 47 must be printed at some point before page number 53.
/// (47 doesn't necessarily need to be immediately before 53; other pages are allowed to be between them.)
///
/// The second section specifies the page numbers of each update.
/// Because most safety manuals are different, the pages needed in the updates are different too.
/// The first update, 75,47,61,53,29, means that the update consists of page numbers 75, 47, 61, 53, and 29.
///
/// To get the printers going as soon as possible, start by identifying which updates are already in the right order.
///
/// In the above example, the first update (75,47,61,53,29) is in the right order:
///
///     75 is correctly first because there are rules that put each other page after it: 75|47, 75|61, 75|53, and 75|29.
///     47 is correctly second because 75 must be before it (75|47) and every other page must be after it according to 47|61, 47|53, and 47|29.
///     61 is correctly in the middle because 75 and 47 are before it (75|61 and 47|61) and 53 and 29 are after it (61|53 and 61|29).
///     53 is correctly fourth because it is before page number 29 (53|29).
///     29 is the only page left and so is correctly last.
///
/// Because the first update does not include some page numbers, the ordering rules involving those missing page numbers are ignored.
///
/// The second and third updates are also in the correct order according to the rules.
/// Like the first update, they also do not include every page number, and so only some of the ordering rules apply
/// - within each update, the ordering rules that involve missing page numbers are not used.
///
/// The fourth update, 75,97,47,61,53, is not in the correct order: it would print 75 before 97, which violates the rule 97|75.
///
/// The fifth update, 61,13,29, is also not in the correct order, since it breaks the rule 29|13.
///
/// The last update, 97,13,75,29,47, is not in the correct order due to breaking several rules.
///
/// For some reason, the Elves also need to know the middle page number of each update being printed.
/// Because you are currently only printing the correctly-ordered updates,
/// you will need to find the middle page number of each correctly-ordered update. In the above example, the correctly-ordered updates are:
///
/// 75,47,61,53,29
/// 97,61,53,29,13
/// 75,29,13
/// These have middle page numbers of 61, 53, and 29 respectively. Adding these page numbers together gives 143.
use std::collections::BTreeMap;
use std::collections::BTreeSet;

const CONTENTS: &str = include_str!("../metadata/input.txt");

type RulesMap = BTreeMap<u32, BTreeSet<u32>>;

pub fn part1() {
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
    for print_line in print_line_list {
        if is_order_correct(&print_line, &rules_map) {
            if let Some(mid_elem) = print_line.get(print_line.len() / 2) {
                println!("mid elem: {mid_elem}");
                result += mid_elem;
            }
        }
    }

    println!("result: {result}");
}

fn is_order_correct(print_line: &[u32], rules_map: &RulesMap) -> bool {
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
                        return false;
                    }
                }
            }
        }
    }

    true
}
