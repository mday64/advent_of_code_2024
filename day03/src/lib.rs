use nom::{branch::alt, bytes::complete::{tag, take, take_while_m_n}, character::complete::anychar, multi::{many0, many1, many_till}, sequence::{preceded, separated_pair, terminated}, IResult};
use nom::AsChar;
use nom::Parser;

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

pub fn part1(input: &str) -> u32 {
    let (_remaining, args) = many0(alt((mul_instruction, ignore_character)))(input).unwrap();
    args.iter()
        .map(|(a, b)| a*b)
        .sum()
}

pub fn part1_many_till(input: &str) -> u32 {
    let (_remaining, args) = many1(
            many_till(anychar, mul_instruction)
            .map(|(_chars, args)| args)
        )(input)
        .unwrap();
    args.iter()
        .map(|(a, b)| a*b)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let mut result = 0;
    let mut remaining = input;
    let mut enabled = true;

    while !remaining.is_empty() {
        if let Ok((r, (a,b))) = mul_instruction(remaining) {
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

pub fn part2_state_machine(input: &str) -> u32 {
    enum States {
        Looking,    // Not inside any function
        Mul1,       // Saw "m"
        Mul2,       // Saw "mu"
        Mul3,       // Saw "mul"
        Mul4,       // Saw "mul("
        Mul5,       // Saw "mul(" and one or more digits
        Mul6,       // Saw "mul(" <digit+> ","
        Mul7,       // Saw "mul(" <digit+> "," <digit+>
        Do1,        // Saw "d"
        Do2,        // Saw "do"
        Do3,        // Saw "do("
        Dont1,      // Saw "don"
        Dont2,      // Saw "don'"
        Dont3,      // Saw "don't"
        Dont4,      // Saw "don't("
    }
    use States::*;

    struct State {
        state: States,
        arg0: u32,      // First argument to mul()
        arg1: u32,      // Second argument to mul()
        enabled: bool,  // If true, process mul() instructions
        sum: u32        // The sum of all enabled mul() instructions
    }

    let mut state = State {
        state: Looking,
        arg0: 0, arg1: 0,
        enabled: true,
        sum: 0
    };

    for ch in input.chars() {
        state.state = match (state.state, ch) {
            (Looking, 'm') => Mul1,
            (Mul1, 'u') => Mul2,
            (Mul2, 'l') => Mul3,
            (Mul3, '(') => { state.arg0 = 0; Mul4 }
            (Mul4, digit) if digit.is_ascii_digit() => {
                state.arg0 = state.arg0 * 10 + (digit as u32 - '0' as u32);
                Mul5
            }
            (Mul5, digit) if digit.is_ascii_digit() => {
                state.arg0 = state.arg0 * 10 + (digit as u32 - '0' as u32);
                Mul5
            }
            (Mul5, ',') => { state.arg1 = 0; Mul6 }
            (Mul6, digit) if digit.is_ascii_digit() => {
                state.arg1 = state.arg1 * 10 + (digit as u32 - '0' as u32);
                Mul7
            }
            (Mul7, digit) if digit.is_ascii_digit() => {
                state.arg1 = state.arg1 * 10 + (digit as u32 - '0' as u32);
                Mul7
            }
            (Mul7, ')') => {
                if state.enabled {
                    state.sum += state.arg0 * state.arg1;
                }
                Looking
            }
            (Looking, 'd') => Do1,
            (Do1, 'o') => Do2,
            (Do2, '(') => Do3,
            (Do3, ')') => { state.enabled = true; Looking }
            (Do2, 'n') => Dont1,
            (Dont1, '\'') => Dont2,
            (Dont2, 't') => Dont3,
            (Dont3, '(') => Dont4,
            (Dont4, ')') => { state.enabled = false; Looking } 
            _ => Looking
        }
    }

    state.sum
}

#[test]
fn test_part1() {
    let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    assert_eq!(part1(input), 161);
    assert_eq!(part1_many_till(input), 161);
}

#[test]
fn test_part2() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    assert_eq!(part2(input), 48);
    assert_eq!(part2_state_machine(input), 48);
}

#[cfg(test)]
const FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full_input() {
    assert_eq!(part1(FULL_INPUT), 181345830);
    assert_eq!(part1_many_till(FULL_INPUT), 181345830);
}

#[test]
fn test_part2_full_input() {
    assert_eq!(part2(FULL_INPUT), 98729041);
    assert_eq!(part2_state_machine(FULL_INPUT), 98729041);
}
