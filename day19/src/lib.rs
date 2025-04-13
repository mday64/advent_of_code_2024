use nom::{bytes::tag, character::complete::alpha1, multi::separated_list1, sequence::separated_pair, Parser, IResult};

fn parse_input(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (remaining, (towels, patterns))= separated_pair(
        separated_list1(tag(", "), alpha1),
        tag("\n\n"),
        separated_list1(tag("\n"), alpha1)
    ).parse_complete(input)?;

    Ok((remaining, (towels, patterns)))
}

fn pattern_from_towels(pattern: &str, towels: &[&str]) -> bool {
    if pattern.is_empty() { return true }
    let result = towels.iter().any(|t| pattern.starts_with(t) && pattern_from_towels(&pattern[t.len()..], towels));
    // eprintln!("pattern: {pattern} -> {result}");
    result
}

pub fn part1(input: &str) -> usize {
    let (_remaining, (towels, patterns)) = parse_input(input).expect("parse");

    patterns.iter().filter(|p| pattern_from_towels(p, &towels)).count()
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
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
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}

#[cfg(test)]
const FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 238);
}
