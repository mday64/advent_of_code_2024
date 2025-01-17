use itertools::{repeat_n, Itertools};
use nom::{bytes::complete::tag, character::complete::u64 as parse_u64, multi::separated_list1, sequence::separated_pair, IResult};

fn parse_operands(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(tag(" "), parse_u64)(input)
}

fn parse_line(input: &str) -> IResult<&str, (u64, Vec<u64>)> {
    separated_pair(parse_u64, tag(": "), parse_operands)(input)
}

#[derive(Debug, Clone, Copy)]
enum Operator { Add, Multiply }

pub fn part1(input: &str) -> u64 {
    use Operator::{Add, Multiply};

    input.lines().filter_map(|line| {
        let (_remaining, (result, operands)) = parse_line(line).unwrap();
        for operators in repeat_n([Add, Multiply], operands.len()-1).multi_cartesian_product() {
            let temp = operands[1..].iter().zip(operators).fold(operands[0], |acc, (operand, operator)| {
                match operator {
                    Add => acc + operand,
                    Multiply => acc * operand,
                }
            });
            if temp == result {
                return Some(result);
            }
        }
        None
    }).sum()
}

pub fn part2(_input: &str) -> u32 {
    43
}

#[cfg(test)]
mod test {
    use super::{part1, part2};

    const FULL_INPUT: &str = include_str!("../input.txt");
    const EXAMPLE_INPUT: &str = "\
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(EXAMPLE_INPUT), 3749);
    }

    #[test]
    fn test_part1_full() {
        assert_eq!(part1(FULL_INPUT), 20665830408335);
    }

    #[test]
    fn test_part2() {
        let input = "Hello, World!";
        assert_eq!(part2(input), 43);
    }
}