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

//
// I think part 2 would be better served by a different data representation.
// Instead of keeping track of individual blocks, keep track of runs of
// blocks belonging to a file or free space.
//
pub fn part2(input: &str) -> u64 {
    type Position = u32;
    type FileID = u16;
    type Length = u8;
    struct FileInfo {
        position: Position,
        file_id: FileID,
        length: Length
    }
    struct FreeSpaceInfo {
        position: Position,
        length: Length
    }
    let mut file_blocks = Vec::<FileInfo>::new();
    let mut free_blocks = Vec::<FreeSpaceInfo>::new();

    // Parse the input
    let input = input.trim_end();
    let input = input.as_bytes();
    let mut position = 0;
    let mut file_id = 0;
    let mut is_free = false;        // Is the current run free space?
    for byte in input {
        if !byte.is_ascii_digit() { break; }
        let length = byte - b'0';
        if is_free {
            free_blocks.push(FreeSpaceInfo{position, length});
        } else {
            file_blocks.push(FileInfo{position, file_id, length});
            file_id += 1;
        }
        position += length as Position;
        is_free = !is_free;
    }

    // Move whole files to the chunk of free space closest to the start
    // that is big enough to contain the file.
    for file in file_blocks.iter_mut().rev() {
        // Try to find some space to move `file`
        for free in free_blocks.iter_mut() {
            if free.position > file.position {
                // No suitable space.  Can't move this file.
                break;
            }
            if free.length < file.length {
                // Look for a longer free run
                continue;
            }

            // Move the file
            file.position = free.position;
            free.position += file.length as u32;
            free.length -= file.length;
            break;
        }
    }

    // Calculate the checksum
    file_blocks.into_iter().map(|file| {
        ((file.position as u64) .. (file.position as u64 + file.length as u64))
        .sum::<u64>() * file.file_id as u64
    })
    .sum()
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

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT), 6390781891880);
}
