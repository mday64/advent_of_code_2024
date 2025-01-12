use day06::{part1, part2};

fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");

    let result2 = part2(input);
    println!("Part 2: {result2}");
}
