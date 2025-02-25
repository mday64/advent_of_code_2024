use std::collections::HashSet;

use nom::{bytes::complete::tag, character::complete::{self,line_ending}, multi::separated_list1, sequence::{preceded, separated_pair}, IResult, Parser};

struct Coord {
    x: i32,
    y: i32
}
struct Robot {
    position: Coord,
    velocity: Coord
}

impl Robot {
    fn update(&mut self) {
        self.position.x = (self.position.x + self.velocity.x).rem_euclid(101);
        self.position.y = (self.position.y + self.velocity.y).rem_euclid(103);
    }
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

pub fn part2_helper(input: &str) {
    let (_rest, mut robots) = separated_list1(line_ending, parse_line).parse(input).expect("valid input");
    for i in 1..10000 {
        let mut grid = [[b' ';101]; 103];
        robots.iter_mut().for_each(Robot::update);
        for robot in robots.iter() {
            grid[robot.position.y as usize][robot.position.x as usize] = b'#';
        }
        println!("\x0cAfter {} seconds:", i);
        for row in grid {
            for ch in row {
                print!("{}", ch as char);
            }
            println!();
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}

pub fn part2() -> u32 {
    // I've noticed that the output of part2_helper() has notable horizontal
    // content for iterations of the form 63 + M * 103, and vertical for
    // 82 + N * 101.  So solve for a number that fits both congruences.

    let verticals: HashSet<u32> = (0..103).map(|i| i*101+82).collect();
    let horizontals: HashSet<u32> = (0..101).map(|i| i*103+63).collect();
    let solution: Vec<u32> = verticals.intersection(&horizontals).cloned().collect();
    assert_eq!(solution.len(), 1);
    solution[0]
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    let (rest,(x,y)) = separated_pair(complete::i32, tag(","), complete::i32).parse(input)?;
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
