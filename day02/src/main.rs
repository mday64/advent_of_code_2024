fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 356);
}

fn report_is_safe(report: &str) -> bool {
    let mut numbers = report.split_whitespace().map(|word| word.parse::<i32>().expect("invalid number"));
    let first = numbers.next().expect("number");
    let mut last = numbers.next().expect("number");
    let sign = (last - first).signum();
    if sign == 0 || (last - first).abs() > 3 {
        return false;
    }

    for n in numbers {
        if (n - last).signum() != sign || (n - last).abs() > 3 {
            return false;
        }
        last = n;
    }

    true
}

fn part1(input: &str) -> usize {
    input.lines().filter(|line| report_is_safe(line)).count()
}

#[test]
fn test_part1() {
    let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    assert_eq!(part1(input), 2);
}