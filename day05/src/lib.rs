use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{newline, u32 as parse_u32},
    combinator::iterator,
    multi::separated_list1,
    sequence::{pair, separated_pair, terminated},
    IResult
};

//
// The task is to check whether the various lists of numbers are sorted
// consistent with the sorting rules given.
//
// In the example, there are rules for all pairs of numbers that appear
// in the lists.  I wonder if that's true for the full problem, or if
// we'll need transitivity to figure it out.
//
// I wonder if the sorting rules lead to a total ordering of those numbers.
// If so, we could just sort them in order, and then see if each input list
// is an (ordered) subset of the total ordering.
//
// The slice type has an is_sorted() method, which requires the PartialOrd
// trait.  Since we would need to use the given sorting rules, we would
// have to wrap the numbers in a different type that could implement
// PartialOrd based on the given sorting rules.  I think it's probably
// easier to implement the is_sorted check ourselves.
//
pub fn part1(input: &str) -> u32 {
    let mut result = 0;
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Parse the ordering rules
    let mut is_sorted = HashMap::<(u32, u32), bool>::new();
    for line in rules.lines() {
        if line.is_empty() { break; }
        let (left, right) = line.split_once('|').unwrap();
        let left: u32 = left.parse().unwrap();
        let right: u32 = right.parse().unwrap();
        is_sorted.insert((left, right), true);
        is_sorted.insert((right, left), false);
    }

    // Parse the lists of numbers
    'update: for line in updates.lines() {
        let pages = line.split(',').map(|word| word.parse::<u32>().unwrap()).collect_vec();
        for (left, right) in pages.iter().tuple_windows() {
            if !(is_sorted.get(&(*left, *right)).unwrap()) { continue 'update; }
        }
        result += pages[pages.len()/2]
    }

    //      If the list is sorted, then:
    //          add its middle element to the result
    result
}

#[test]
fn test_part1() {
    let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(part1(input), 143);
}

//
// Sort the list of page numbers based on the given sorting rules.
// Returns true if the list was modified.
//
fn sort_pages(pages: &mut[u32], is_sorted: &HashMap<(u32, u32), bool>) -> bool {
    let mut result = false;
    let mut changed = true;

    // A simple, poorly optimized, bubble sort
    while changed {
        changed = false;
        for i in 1..pages.len() {
            let left = pages[i-1];
            let right = pages[i];
            if !is_sorted.get(&(left, right)).unwrap() {
                changed = true;
                result = true;
                pages.swap(i-1, i);
            }
        }
    }

    result
}

pub fn part2(input: &str) -> u32 {
    let mut result = 0;
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Parse the ordering rules
    let mut is_sorted = HashMap::<(u32, u32), bool>::new();
    for line in rules.lines() {
        if line.is_empty() { break; }
        let (left, right) = line.split_once('|').unwrap();
        let left: u32 = left.parse().unwrap();
        let right: u32 = right.parse().unwrap();
        is_sorted.insert((left, right), true);
        is_sorted.insert((right, left), false);
    }

    // Parse the lists of numbers
    for line in updates.lines() {
        // dbg!(line);
        let mut pages = line.split(',').map(|word| word.parse::<u32>().unwrap()).collect_vec();
        if sort_pages(&mut pages, &is_sorted) {
            // They were not in sorted order, so add the new middle element to result
            // dbg!(&pages);
            result += pages[pages.len()/2]
        }
    }

    result
}

#[test]
fn test_part2() {
    let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(part2(input), 123);
}

pub fn both_parts(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Parse the ordering rules
    let mut is_sorted = HashMap::<(u32, u32), bool>::new();
    for line in rules.lines() {
        if line.is_empty() { break; }
        let (left, right) = line.split_once('|').unwrap();
        let left: u32 = left.parse().unwrap();
        let right: u32 = right.parse().unwrap();
        is_sorted.insert((left, right), true);
        is_sorted.insert((right, left), false);
    }

    // Parse the lists of numbers
    for line in updates.lines() {
        // dbg!(line);
        let mut pages = line.split(',').map(|word| word.parse::<u32>().unwrap()).collect_vec();
        if sort_pages(&mut pages, &is_sorted) {
            // They were not in sorted order, so add the new middle element to result
            // dbg!(&pages);
            part2 += pages[pages.len()/2]
        } else {
            part1 += pages[pages.len()/2]
        }
    }

    (part1, part2)
}


#[test]
fn test_both_parts() {
    let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(both_parts(input), (143, 123));
}


pub fn both_parts_faster(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Parse the ordering rules
    let mut is_sorted = HashMap::<(u32, u32), bool>::new();
    for line in rules.lines() {
        if line.is_empty() { break; }
        let (left, right) = line.split_once('|').unwrap();
        let left: u32 = left.parse().unwrap();
        let right: u32 = right.parse().unwrap();
        is_sorted.insert((left, right), true);
        is_sorted.insert((right, left), false);
    }

    let compare_pages = |left: &u32, right: &u32| -> Ordering{
        if left == right {
            Ordering::Equal
        } else if *is_sorted.get(&(*left, *right)).unwrap() {
                Ordering::Less
        } else {
            Ordering::Greater
        }
    };

    // Parse the lists of numbers
    for line in updates.lines() {
        // dbg!(line);
        let mut pages = line.split(',').map(|word| word.parse::<u32>().unwrap()).collect_vec();
        if pages.is_sorted_by(|left, right| *is_sorted.get(&(*left, *right)).unwrap()) {
            // They were not in sorted order, so add the new middle element to result
            // dbg!(&pages);
            part1 += pages[pages.len()/2]
        } else {
            pages.sort_by(compare_pages);
            part2 += pages[pages.len()/2]
        }
    }

    (part1, part2)
}


#[test]
fn test_both_parts_faster() {
    let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(both_parts_faster(input), (143, 123));
}

pub fn both_parts_hashset(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Parse the ordering rules
    let mut is_sorted = HashSet::<(u32, u32)>::new();
    for line in rules.lines() {
        if line.is_empty() { break; }
        let (left, right) = line.split_once('|').unwrap();
        let left: u32 = left.parse().unwrap();
        let right: u32 = right.parse().unwrap();
        is_sorted.insert((left, right));
    }

    let compare_pages = |left: &u32, right: &u32| -> Ordering{
        if left == right {
            Ordering::Equal
        } else if is_sorted.contains(&(*left, *right)) {
                Ordering::Less
        } else {
            Ordering::Greater
        }
    };

    // Parse the lists of numbers
    for line in updates.lines() {
        // dbg!(line);
        let mut pages = line.split(',').map(|word| word.parse::<u32>().unwrap()).collect_vec();
        if pages.is_sorted_by(|left, right| is_sorted.contains(&(*left, *right))) {
            // They were in sorted order, so add the new middle element to part1
            // dbg!(&pages);
            part1 += pages[pages.len()/2]
        } else {
            // They were not sorted, so sort and add to part2
            pages.sort_by(compare_pages);
            part2 += pages[pages.len()/2]
        }
    }

    (part1, part2)
}

#[test]
fn test_both_parts_hashset() {
    let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(both_parts_hashset(input), (143, 123));
}

type Rule = (u32, u32);
fn parse_rule(input: &str) -> IResult<&str, Rule> {
    terminated(separated_pair(parse_u32, tag("|"), parse_u32), newline)(input)
}

type Rules = HashSet<Rule>;
fn parse_rules(input: &str) -> IResult<&str, Rules> {
    let mut it = iterator(input, parse_rule);
    let rules: Rules = it.collect();
    let (remainder, _) = it.finish()?;
    let (remainder, _newline) = newline(remainder)?;
    Ok((remainder, rules))
}

type PageList = Vec<u32>;
fn parse_page_list(input: &str) -> IResult<&str, PageList> {
    separated_list1(tag(","), parse_u32)(input)
}

// TODO: Could this be an iterator of PageList's?
type PageLists = Vec<PageList>;
fn parse_page_lists(input: &str) -> IResult<&str, PageLists> {
    separated_list1(newline, parse_page_list)(input)
}

fn parse_input(input: &str) -> IResult<&str, (Rules, PageLists)> {
    pair(parse_rules, parse_page_lists)(input)
}

pub fn both_parts_nom(input: &str) -> (u32, u32) {
    let mut part1 = 0;
    let mut part2 = 0;
    let (_, (is_sorted, page_lists)) = parse_input(input).unwrap();

    let compare_pages = |left: &u32, right: &u32| -> Ordering{
        if left == right {
            Ordering::Equal
        } else if is_sorted.contains(&(*left, *right)) {
                Ordering::Less
        } else {
            Ordering::Greater
        }
    };

    for mut pages in page_lists {
        if pages.is_sorted_by(|left, right| is_sorted.contains(&(*left, *right))) {
            // They were in sorted order, so add the new middle element to part1
            // dbg!(&pages);
            part1 += pages[pages.len()/2]
        } else {
            // They were not sorted, so sort and add to part2
            pages.sort_by(compare_pages);
            part2 += pages[pages.len()/2]
        }
    }

    (part1, part2)
}

#[test]
fn test_both_parts_nom() {
    let input = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";
    assert_eq!(both_parts_nom(input), (143, 123));
}
