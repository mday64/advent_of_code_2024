pub fn part1(input: &str) -> u64 {
    input.lines()
        .map(|line| line.parse::<u64>().expect("u64"))
        .map(|num| {
            let mut result = num;
            for _ in 0..2000 {
                result = next_secret_number(result);
            }
            result
        })
        .sum()
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}


fn next_secret_number(secret: u64) -> u64 {
    let mut result = (secret ^ (secret << 6)) & 16777215;
    result = (result ^ (result >> 5)) & 16777215;
    result = (result ^ (result << 11)) & 16777215;
    result
}

#[test]
fn test_next_secret_number() {
    assert_eq!(next_secret_number(123),      15887950);
    assert_eq!(next_secret_number(15887950), 16495136);
    assert_eq!(next_secret_number(16495136),   527345);
    assert_eq!(next_secret_number(527345),     704524);
    assert_eq!(next_secret_number(704524),    1553684);
    assert_eq!(next_secret_number(1553684),  12683156);
    assert_eq!(next_secret_number(12683156), 11100544);
    assert_eq!(next_secret_number(11100544), 12249484);
    assert_eq!(next_secret_number(12249484),  7753432);
    assert_eq!(next_secret_number(7753432),   5908254);
}

#[test]
fn test_part1() {
    assert_eq!(part1("1\n"), 8685429);
    assert_eq!(part1("10\n"), 4700978);
    assert_eq!(part1("100\n"), 15273692);
    assert_eq!(part1("2024\n"), 8667524);
    assert_eq!(part1("1\n10\n100\n2024\n"), 8685429+4700978+15273692+8667524);
}

#[test]
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}
