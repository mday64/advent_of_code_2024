pub fn part1(input: &str) -> usize {
    let mut numbers: Vec<u64> = input
        .trim_end()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    for _ in 0..25 {
        numbers = part1_iteration(&numbers);
    }

    numbers.len()
}

fn part1_iteration(numbers: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    for number in numbers {
        match number {
            0 => result.push(1),
            10..=99 => { result.push(number/10); result.push(number%10); }
            1000..=9999 => { result.push(number/100); result.push(number%100); }
            100000..=999999 => { result.push(number/1000); result.push(number%1000); }
            10000000..=99999999 => { result.push(number/10000); result.push(number%10000); }
            1000000000..=9999999999 => { result.push(number/100000); result.push(number%100000); }
            100000000000..=999999999999 => { result.push(number/1000000); result.push(number%1000000); }
            10000000000000..=99999999999999 => { result.push(number/10000000); result.push(number%10000000); }
            1000000000000000..=9999999999999999 => { result.push(number/100000000); result.push(number%100000000); }
            100000000000000000..=999999999999999999 => { result.push(number/1000000000); result.push(number%1000000000); }
            10000000000000000000.. => { result.push(number/10000000000); result.push(number%10000000000); }
            _ => result.push(number * 2024)
        }
    }
    result
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

#[test]
fn test_part1() {
    assert_eq!(part1("125 17"), 55312);
}

#[test]
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
