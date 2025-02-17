use nom::{bytes::complete::tag, character::complete::{multispace0, i64}, multi::many1, IResult, Parser};

//
// Part 1
//
// This appears to be solving a system of simultaneous linear equations.
// If you press button "A" `m` times, and button "B" `n` times, then
// A.x * m + B.x * n = Prize.x
// A.y * m + B.y * n = Prize.y
// => solve for m, n (for m,n in 0..=100)
//
//     B.y*P.x - B.x*P.y
// m = -----------------
//     B.y*A.x - B.x*A.y
//
//     A.y*P.x - A.x*P.y   A.x*P.y - A.y*P.x
// n = ----------------- = -----------------
//     A.y*B.x - A.x*B.y   B.y*A.x - B.x*A.y
//
// If there is a solution, then the cost is 3 * m + n.
// Sum the costs of all possible solutions.
//
struct Coord {
    x: i64,
    y: i64
}
fn parse_button_a(input: &str) -> IResult<&str, Coord> {
    let (input, _) = tag("Button A: X+")(input)?;
    let (input, x) = i64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = i64(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Coord{x,y}))
}

fn parse_button_b(input: &str) -> IResult<&str, Coord> {
    let (input, _) = tag("Button B: X+")(input)?;
    let (input, x) = i64(input)?;
    let (input, _) = tag(", Y+")(input)?;
    let (input, y) = i64(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Coord{x,y}))
}

fn parse_prize(input: &str) -> IResult<&str, Coord> {
    let (input, _) = tag("Prize: X=")(input)?;
    let (input, x) = i64(input)?;
    let (input, _) = tag(", Y=")(input)?;
    let (input, y) = i64(input)?;
    let (input, _) = multispace0(input)?;
    Ok((input, Coord{x,y}))
}

struct Machine {
    a: Coord,
    b: Coord,
    prize: Coord
}
fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a, b, prize)) = (parse_button_a, parse_button_b, parse_prize).parse(input)?;
    Ok((input, Machine{a, b, prize}))
}

fn parse_machines(input: &str) -> IResult<&str, Vec<Machine>> {
    many1(parse_machine).parse(input)
}

pub fn part1(input: &str) -> i64 {
    let (_, machines) = parse_machines(input).expect("well formed input");
    machines.iter().filter_map(|m| {
        let denominator = m.b.y * m.a.x - m.b.x * m.a.y;
        let num_m = m.b.y * m.prize.x - m.b.x * m.prize.y;
        let num_n = m.a.x * m.prize.y - m.a.y * m.prize.x;
    
        if num_m % denominator == 0 && num_n % denominator == 0 {
            let m = num_m / denominator;
            let n = num_n / denominator;
            if m >= 0 && n >= 0 {
                return Some(3*m+n);
            }
        }
        None
    }).sum()
}

pub fn part2(input: &str) -> i64 {
    let (_, machines) = parse_machines(input).expect("well formed input");
    machines.iter().filter_map(|m| {
        let px = m.prize.x + 10000000000000;
        let py = m.prize.y + 10000000000000;
        let denominator = m.b.y * m.a.x - m.b.x * m.a.y;
        let num_m = m.b.y * px - m.b.x * py;
        let num_n = m.a.x * py - m.a.y * px;
    
        if num_m % denominator == 0 && num_n % denominator == 0 {
            let m = num_m / denominator;
            let n = num_n / denominator;
            if m >= 0 && n >= 0 {
                return Some(3*m+n);
            }
        }
        None
    }).sum()
}

#[test]
fn test_part1() {
    let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    assert_eq!(part1(input), 480);
}

#[test]
fn test_part2() {
    let input = "\
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
    assert_eq!(part2(input), 875318608908);
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 26810);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 108713182988244);
}
