use day05::both_parts;

fn main() {
    let input = include_str!("../input.txt");

    let (result1, result2) = both_parts(input);
    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
