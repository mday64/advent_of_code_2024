fn main() {
    let input = include_str!("../input.txt");

    let result1 = part1(input);
    println!("Part 1: {result1}");
    assert_eq!(result1, 356);

    let result2 = part2(input);
    println!("Part 2: {result2}");
    assert_eq!(result2, 413);
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

fn find_bad_level(levels: &[i32]) -> Option<usize> {
    let sign = (levels[1] - levels[0]).signum();
    if sign == 0 {
        return Some(1)
    }
    for i in 1..levels.len() {
        if (levels[i] - levels[i-1]).signum() != sign || (levels[i] - levels[i-1]).abs() > 3 {
            return Some(i)
        }
    }
    None
}

//
// If we find a level that is inconsistent (according to the rules for part 1),
// we need to try again after removing either that level, or the preceding one.
//
fn report_is_safe2(report: &str) -> bool {
    let mut levels: Vec<i32> = report.split_whitespace().map(|w| w.parse().expect("parse")).collect();
    if let Some(i) = find_bad_level(&levels) {
        // Try removing levels[0]
        if find_bad_level(&levels[1..]).is_none() {
            return true;
        }
        // Try removing levels[i]
        let misfit = levels.remove(i);
        if find_bad_level(&levels).is_none() {
            return true;
        }
        // Try removing levels[i-1]
        levels.insert(i, misfit);
        levels.remove(i-1);
        find_bad_level(&levels).is_none()
    } else {
        true
    }
}

fn part2(input: &str) -> usize {
    // input.lines().filter(|line| report_is_safe2(line)).count()
    let mut result = 0;
    for (i, report) in input.lines().enumerate() {
        let safe = report_is_safe2(report);
        if safe {
            result += 1;
        }
        eprintln!("{}: {safe}", i+1);
    }
    result
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

#[test]
fn test_part2() {
    let input = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    assert_eq!(part2(input), 4);
}

#[test]
#[allow(clippy::bool_assert_comparison)]
fn test_report_is_safe2() {
    assert_eq!(report_is_safe2("7 6 4 2 1"), true);
    assert_eq!(report_is_safe2("1 2 7 8 9"), false);
    assert_eq!(report_is_safe2("9 7 6 2 1"), false);
    assert_eq!(report_is_safe2("1 3 2 4 5"), true);     // Remove "3"
    assert_eq!(report_is_safe2("8 6 4 4 1"), true);     // Remove either "4"
    assert_eq!(report_is_safe2("1 3 6 7 9"), true);
    assert_eq!(report_is_safe2("1 2 9 3 4"), true);     // Remove "9"
    assert_eq!(report_is_safe2("76 77 79 82 84 87 89 95"), true);   // Remove "95"
    assert_eq!(report_is_safe2("41 38 40 42 44 47"), true);     // Remove "41"
}
