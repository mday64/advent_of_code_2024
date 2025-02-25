use nom::{bytes::complete::tag, character::complete::{line_ending, i32}, multi::separated_list1, sequence::{preceded, separated_pair}, IResult, Parser};

struct Coord {
    x: i32,
    y: i32
}
struct Robot {
    position: Coord,
    velocity: Coord
}

pub fn part1_with_size(input: &str, width: i32, height: i32) -> usize {
    let (_rest, robots) = separated_list1(line_ending, parse_line).parse(input).expect("valid input");
    let mut upper_left = 0;
    let mut upper_right = 0;
    let mut lower_left = 0;
    let mut lower_right = 0;

    for robot in robots {
        let x = (robot.position.x + 100 * robot.velocity.x).rem_euclid(width);
        let y = (robot.position.y + 100 * robot.velocity.y).rem_euclid(height);
        if y < height/2 && x < width/2 {
            upper_left += 1;
        } else if y < height/2 && x > width/2 {
            upper_right += 1;
        } else if y > height/2 && x < width/2 {
            lower_left += 1;
        } else if y> height/2 && x > width/2 {
            lower_right += 1;
        }
    }

    upper_left * upper_right * lower_left * lower_right
}

pub fn part1(input: &str) -> usize {
    part1_with_size(input, 101, 103)
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    let (rest,(x,y)) = separated_pair(i32, tag(","), i32).parse(input)?;
    Ok((rest, Coord{x,y}))
}

fn parse_line(input: &str) -> IResult<&str, Robot> {
    separated_pair(
        preceded(tag("p="), parse_coord),
        tag(" "),
        preceded(tag("v="), parse_coord)
    ).parse(input)
    .map(|(rest, (position, velocity))| (rest, Robot{position, velocity}))
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1() {
    let input = "\
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";
    assert_eq!(part1_with_size(input, 11, 7), 12);
}

#[test]
fn part1_full() {
    assert_eq!(part1(FULL_INPUT), 222901875);
}

#[test]
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
