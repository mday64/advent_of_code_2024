use std::collections::HashMap;

use itertools::Itertools;

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