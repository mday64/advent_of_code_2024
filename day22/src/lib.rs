use std::collections::HashMap;
use itertools::Itertools;

pub fn part1(input: &str) -> u64 {
    input.lines()
        .map(|line| line.parse::<u64>().expect("u64"))
        .map(|num| {
            let mut result = num;
            for _ in 0..2000 {
                result = next_secret_number(result);
            }
            result
        })
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let buyers: Vec<_> = input.lines()
        .map(|line| line.parse::<u64>().expect("u64"))
        .map(prices_and_changes)
        .collect();
    
    // Build a map of all unique sequences of 4 price changes to the list
    // of prices for those changes.
    let mut all_price_changes = HashMap::new();
    for buyer in &buyers {
        let mut buyer_first_price_changes = HashMap::new();
        // Insert the FIRST price for any sequence of changes
        for ((_,a), (_,b), (_,c), (price,d)) in buyer.iter().tuple_windows() {
            buyer_first_price_changes.entry((a, b, c, d)).or_insert(price);
        }

        // Merge this buyer's prices into all_price_changes
        for (changes, price) in buyer_first_price_changes {
            *all_price_changes.entry(changes).or_insert(0u32) += *price as u32;
        }
    }

    // For every sequence of price changes, find the price (if any)
    // associated with the first occurence of that sequence of changes.
    *all_price_changes.values().max().unwrap()
}

fn next_secret_number(secret: u64) -> u64 {
    let mut result = (secret ^ (secret << 6)) & 16777215;
    result = (result ^ (result >> 5)) & 16777215;
    result = (result ^ (result << 11)) & 16777215;
    result
}

type Price = i8;
type PriceChange = i8;

fn prices_and_changes(initial: u64) -> Vec<(Price, PriceChange)> {
    let mut last_secret = initial;
    let mut last_price = (last_secret % 10) as Price;
    (0..2000).map(|_| {
            let secret = next_secret_number(last_secret);
            let price = (secret % 10) as Price;
            let change = price - last_price;
            let item = (price, change);
            last_secret = secret;
            last_price = price;
            item
    })
    .collect()
}

#[test]
fn test_next_secret_number() {
    assert_eq!(next_secret_number(123),      15887950);
    assert_eq!(next_secret_number(15887950), 16495136);
    assert_eq!(next_secret_number(16495136),   527345);
    assert_eq!(next_secret_number(527345),     704524);
    assert_eq!(next_secret_number(704524),    1553684);
    assert_eq!(next_secret_number(1553684),  12683156);
    assert_eq!(next_secret_number(12683156), 11100544);
    assert_eq!(next_secret_number(11100544), 12249484);
    assert_eq!(next_secret_number(12249484),  7753432);
    assert_eq!(next_secret_number(7753432),   5908254);
}

#[test]
fn test_part1() {
    assert_eq!(part1("1\n"), 8685429);
    assert_eq!(part1("10\n"), 4700978);
    assert_eq!(part1("100\n"), 15273692);
    assert_eq!(part1("2024\n"), 8667524);
    assert_eq!(part1("1\n10\n100\n2024\n"), 8685429+4700978+15273692+8667524);
}

#[test]
fn test_part2() {
    let input = "1\n2\n3\n2024\n";
    assert_eq!(part2(input), 23);
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 14392541715);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 1628);
}
