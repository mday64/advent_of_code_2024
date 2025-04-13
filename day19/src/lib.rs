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

pub fn part2(input: &str) -> usize {
    fn pattern_from_towels(pattern: &str, towels: &[&str]) -> usize {
        if pattern.is_empty() { return 1 }
        towels.iter().map(|t|
            if let Some(tail) = pattern.strip_prefix(t) {
                pattern_from_towels(tail, towels)
            } else {
                0
            }
        ).sum()
    }

    let (_remaining, (towels, patterns)) = parse_input(input).expect("parse");
    patterns.iter().map(|p| pattern_from_towels(p, &towels)).sum()
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

#[cfg(test)]
const FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 238);
}
