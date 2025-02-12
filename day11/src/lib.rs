use std::collections::HashMap;

pub fn part1(input: &str) -> usize {
    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();
    let seeds = input
        .trim_end()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap());

    seeds.map(|seed| expanded_length(seed, 25, &mut cache)).sum()
}

fn iterate_one_number(number: u64) -> (u64, Option<u64>) {
    match number {
        0 => (1, None),
        10..=99 => (number/10, Some(number%10)),
        1000..=9999 => (number/100, Some(number%100)),
        100000..=999999 => (number/1000, Some(number%1000)),
        10000000..=99999999 => (number/10000, Some(number%10000)),
        1000000000..=9999999999 => (number/100000, Some(number%100000)),
        100000000000..=999999999999 => (number/1000000, Some(number%1000000)),
        10000000000000..=99999999999999 => (number/10000000, Some(number%10000000)),
        1000000000000000..=9999999999999999 => (number/100000000, Some(number%100000000)),
        100000000000000000..=999999999999999999 => (number/1000000000, Some(number%1000000000)),
        10000000000000000000.. => (number/10000000000, Some(number%10000000000)),
        _ => (number * 2024, None)
    }
}

fn expanded_length(number: u64, iterations: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    if iterations == 0 {
        return 1;
    }
    if let Some(result) = cache.get(&(number, iterations)) {
        return *result;
    }

    let result = match iterate_one_number(number) {
        (num1, None) => expanded_length(num1, iterations-1, cache),
        (num1, Some(num2)) => expanded_length(num1, iterations-1, cache) + expanded_length(num2, iterations-1, cache),
    };
    cache.insert((number, iterations), result);
    result
}

pub fn part2(input: &str) -> usize {
    let mut cache: HashMap<(u64, usize), usize> = HashMap::new();
    let seeds = input
        .trim_end()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap());

    seeds.map(|seed| expanded_length(seed, 75, &mut cache)).sum()
}

#[test]
fn test_part1() {
    assert_eq!(part1("125 17"), 55312);
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 209412);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 248967696501656);
}
