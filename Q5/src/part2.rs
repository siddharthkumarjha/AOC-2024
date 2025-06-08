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
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque};

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
    for print_line in print_line_list {
        if let Some(sorted_line) = topo_sort(&print_line, &rules_map) {
            if sorted_line != print_line {
                if let Some(mid_elem) = sorted_line.get(sorted_line.len() / 2) {
                    result += mid_elem;
                }
            }
        } else {
            println!("cycle detected");
        }
    }

    println!("result: {result}");
}

fn topo_sort(print_line: &[u32], rules_map: &RulesMap) -> Option<Vec<u32>> {
    let mut adj_list: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut in_degree: BTreeMap<u32, usize> = BTreeMap::new();

    for &pages in print_line {
        adj_list.entry(pages).or_default();
        in_degree.entry(pages).or_insert(0);
    }

    // T page -> inf page
    // T page -> inf page
    // inc degree of these inf pages
    for page in print_line {
        if let Some(rules) = rules_map.get(page) {
            for &inferior_page in rules {
                if print_line.contains(&inferior_page)
                    && adj_list.get(page).unwrap().contains(&inferior_page) == false
                {
                    adj_list.get_mut(page).unwrap().insert(inferior_page);
                    *in_degree.get_mut(&inferior_page).unwrap() += 1;
                }
            }
        }
    }

    // Topo logical sort, khan's algorithm
    // push everything of degree 0 in queue, pop it out of q and in the process decrease the degree
    // of dependent edges. repeat the process and you have a topo sorted list
    let mut queue: VecDeque<u32> = VecDeque::new();
    for (&page, &deg) in &in_degree {
        if deg == 0 {
            queue.push_back(page);
        }
    }

    let mut sorted_order: Vec<u32> = Vec::new();

    while let Some(page) = queue.pop_front() {
        sorted_order.push(page);
        if let Some(neighbours) = adj_list.get(&page) {
            for &neighbour in neighbours {
                let deg = in_degree.get_mut(&neighbour).unwrap();
                *deg -= 1;
                if *deg == 0 {
                    queue.push_back(neighbour);
                }
            }
        }
    }

    if sorted_order.len() == print_line.len() {
        Some(sorted_order)
    } else {
        None
    }
}
