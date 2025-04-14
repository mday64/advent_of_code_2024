use std::collections::HashMap;

use nom::{bytes::tag, character::complete::alpha1, multi::separated_list1, sequence::separated_pair, Parser, IResult};

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (remaining, (towels, patterns))= separated_pair(
        separated_list1(tag(", "), alpha1),
        tag("\n\n"),
        separated_list1(tag("\n"), alpha1)
    ).parse_complete(input)?;

    Ok((remaining, (towels, patterns)))
}

pub fn part1(input: &str) -> usize {
    fn pattern_from_towels(pattern: &str, towels: &[&str]) -> bool {
        if pattern.is_empty() { return true }
        towels.iter().any(|t| pattern.starts_with(t) && pattern_from_towels(&pattern[t.len()..], towels))
    }

    let (_remaining, (towels, patterns)) = parse_input(input).expect("parse");
    patterns.iter().filter(|p| pattern_from_towels(p, &towels)).count()
}

//
// A naive recursive solution is far too slow.
//
// Let's try a dynamic programming approach.
//
pub fn part2_dynamic(input: &str) -> usize {
    let (_remaining, (towels, patterns)) = parse_input(input).expect("parse");
    patterns.iter().map(|p| {
        let mut cache = HashMap::<&str,usize>::new();
        for i in (0..p.len()).rev() {
            let suffix = &p[i..];
            let mut combos = 0;
            for towel in &towels {
                if let Some(tail) = suffix.strip_prefix(towel) {
                    combos += if tail.is_empty() {
                        1
                    } else {
                        *cache.get(&tail).unwrap_or(&0)
                    }
                }
            }
            cache.insert(suffix, combos);
        }
        *cache.get(p).unwrap()
    }).sum()
}

pub fn part2_memoize(input: &str) -> usize {
    fn pattern_from_towels<'a>(pattern: &'a str, towels: &[&str], cache: &mut HashMap<&'a str,usize>) -> usize {
        if pattern.is_empty() {
            return 1;
        }
        if let Some(&v) = cache.get(pattern) {
            return v;
        }

        let v = towels.iter().map(|t|
            if let Some(tail) = pattern.strip_prefix(t) {
                pattern_from_towels(tail, towels, cache)
            } else {
                0
            }
        ).sum();
        cache.insert(pattern, v);
        v
    }

    let (_remaining, (towels, patterns)) = parse_input(input).expect("parse");
    let mut cache = HashMap::default();
    patterns.iter().map(|p| pattern_from_towels(p, &towels, &mut cache)).sum()
}

pub use part2_memoize as part2;

#[test]
fn test_part1() {
    let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
    assert_eq!(part1(input), 6);
}

#[test]
fn test_part2() {
    let input = "\
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";
    assert_eq!(part2(input), 16);
}

#[test]
fn test_part2_overlapping() {
    let input = "\
r, wr, b, g, bwu, rb, gb, br

gbr
";
    // The possibilities are:
    //      g, b, r
    //      gb, r
    //      g, br
    // Note that we can't break it down into either
    //      (g) * (br)
    //      (gb) * r
    // both of which would return 2
    assert_eq!(part2(input), 3);
}

#[cfg(test)]
const FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 238);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 635018909726691);
}
