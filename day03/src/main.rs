use std::error::Error;
use nom::{branch::alt, bytes::complete::{tag, take, take_while_m_n}, multi::many0, sequence::{preceded, separated_pair, terminated}, IResult};
use nom::AsChar;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 181345830);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 98729041);

    Ok(())
}

fn short_number(input: &str) -> IResult<&str, u32> {
    // TODO: It seems like there ought to be a way to do this without the closure
    // (by just passing a function of the appropriate type).
    let (remainder, num_str) = take_while_m_n(1, 3, |c:char| c.is_dec_digit())(input)?;
    Ok((remainder, num_str.parse().unwrap()))
}

fn mul_instruction(input: &str) -> IResult<&str, (u32, u32)> {
    preceded(tag("mul("), terminated(separated_pair(short_number, tag(","), short_number), tag(")")))(input)
}

fn ignore_character(input: &str) -> IResult<&str, (u32, u32)> {
    let (remaining, _ignored) = take(1u32)(input)?;
    Ok((remaining, (0, 0)))
}

fn parse_do(input: &str) -> IResult<&str, ()> {
    let (remaining, _found) = tag("do()")(input)?;
    Ok((remaining, ()))
}

fn parse_dont(input: &str) -> IResult<&str, ()> {
    let (remaining, _found) = tag("don't()")(input)?;
    Ok((remaining, ()))
}

fn part1(input: &str) -> u32 {
    let (_remaining, args) = many0(alt((mul_instruction, ignore_character)))(input).unwrap();
    args.iter()
        .map(|(a, b)| a*b)
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut result = 0;
    let mut remaining = input;
    let mut enabled = true;

    while !remaining.is_empty() {
        if let Ok((r, (a,b))) = mul_instruction(remaining) {
            eprintln!("mul(): {a} {b}{}", if enabled { "" } else { " (disabled)"});
            remaining = r;
            if enabled {
                result += a * b;
            }
        } else if let Ok((r, _ignored)) = parse_do(remaining) {
            remaining = r;
            enabled = true;
        } else if let Ok((r, _ignored)) = parse_dont(remaining) {
            remaining = r;
            enabled = false;
        } else {
            remaining = &remaining[1..];
        }
    }

    result
}

#[test]
fn test_part1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(input), 161);
}

#[test]
fn test_part2() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(input), 48);
}
