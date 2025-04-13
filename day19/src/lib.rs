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
pub fn part2(input: &str) -> usize {
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
