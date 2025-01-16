use day06::both_parts_cached;

fn main() {
    let input = include_str!("../input.txt");

    // let result1 = part1(input);
    // println!("Part 1: {result1}");

    // let result2 = part2(input);
    // println!("Part 2: {result2}");

    let (result1, result2) = both_parts_cached(input);
    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
