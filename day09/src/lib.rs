use std::iter::repeat_n;

//
// How do we want to represent which blocks are occupied by which file ID?
// The input is 19,999 digits.  If each of those were a "9", that would
// mean a total of 19,999 * 9 = 179,991 blocks.  If also means that there
// are 9,999 file IDs (0..=9_998).  No run of blocks from the same file ID
// can be longer than 9.
//
// Note that the algorithm can split the blocks of a file over multiple
// runs of free space.
//
// I suppose the brute force way is to have a Vec<u16> where u16 is the
// file ID, or a sentenel value for "free."
//
pub fn part1(input: &str) -> u64 {
    const FREE_SPACE: u16 = u16::MAX;
    let mut blocks = Vec::<u16>::new();

    // Parse the input
    let input = input.trim_end();
    let input = input.as_bytes();
    let mut file_id = 0;
    let mut is_free = false;        // Is the current run free space?
    for byte in input {
        if !byte.is_ascii_digit() { break; }
        let length = byte - b'0';
        if is_free {
            blocks.extend(repeat_n(FREE_SPACE, length as usize));
        } else {
            blocks.extend(repeat_n(file_id, length as usize));
            file_id += 1;
        }
        is_free = !is_free;
    }

    // Shuffle the blocks around
    let mut front = 0;
    let mut back = blocks.len() - 1;
    while front < back {
        if blocks[back] == FREE_SPACE {
            back -= 1;
        } else if blocks[front] != FREE_SPACE {
            front += 1;
        } else {
            blocks[front] = blocks[back];
            blocks[back] = FREE_SPACE;
        }
    }

    // Calculate the checksum
    blocks.into_iter().enumerate().filter_map(|(position, file_id)| {
        if file_id == FREE_SPACE {
            None
        } else {
            Some(position as u64 * file_id as u64)
        }
    }).sum()
}

pub fn part2(_input: &str) -> u64 {
    todo!()
}

#[cfg(test)]
const FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1() {
    let input = "2333133121414131402";
    assert_eq!(part1(input), 1928);
}

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 6367087064415);
}

#[test]
fn test_part2() {
    let input = "2333133121414131402";
    assert_eq!(part2(input), 2858);
}
