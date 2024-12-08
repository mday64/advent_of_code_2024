fn main() {
    let input = include_str!("../input.txt");
    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 2375403);
}

fn part1(input: &str) -> i32 {
    let mut left = Vec::<i32>::new();
    let mut right = Vec::<i32>::new();
    for line in input.lines() {
        let mut words = line.split_whitespace();
        left.push(words.next().expect("invalid input").parse().expect("invalid number"));
        right.push(words.next().expect("invalid input").parse().expect("invalid number"));
    }
    left.sort();
    right.sort();
    left.iter().zip(right.iter()).map(|(l,r)| (l-r).abs()).sum()
}

#[test]
fn test_part1() {
    let input = "\
3   4
4   3
2   5
1   3
3   9
3   3
";
    assert_eq!(part1(input), 11);
}
