pub fn part1(_input: &str) -> String {
    "Hello".to_string()
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

#[test]
fn test_part1() {
    let input = "Hello, World!";
    assert_eq!(part1(input), "Hello");
}

#[test]
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
