//
// The goal is to count how many unique pairs of lock and key do not
// overlap.
//
// The problem talks about the "height" or length in each column, but
// that's not necessary to solve part 1.  All we need to do is check
// for overlapping '#' characters.  We don't even need to split objects
// into key vs. lock; keys will always overlap with keys, and locks
// will always overlap with locks.
//
// Each key/lock is going to be represented by an integer (turning the
// characters of the input into a bitmap).  The test for overlap is
// bitwise AND.
//
// We will take advantage of the fact that b'#' is odd, while b'.'
// and b'\n' are even.  We'll build a bitmap of the input characters,
// using the least significant bit of each character.  Note that each
// key/lock occupies 43 bytes in the input (including the blank line
// at the end of each one).
//
pub fn part1(input: &str) -> u32 {
    let items:Vec<u64> = input.as_bytes().chunks(43).map(|chunk| {
        chunk[0..42].iter().fold(0u64, |v, byte| v * 2 + (byte & 1) as u64)
    }).collect();

    let mut result = 0;
    for i in 0..items.len()-1 {
        for j in (i+1)..items.len() {
            if items[i] & items[j] == 0 {
                result += 1;
            }
        }
    }

    result
}

pub fn part2(_input: &str) -> String {
    "World".to_string()
}

#[test]
fn test_part1() {
    let input = "\
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";
    assert_eq!(part1(input), 3);
}

#[test]
fn test_part2() {
    let input = "Hello, World!";
    assert_eq!(part2(input), "World");
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 3107);
}
