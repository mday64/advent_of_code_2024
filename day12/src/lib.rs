use std::collections::HashSet;
use indexmap::IndexMap;

//
// Find regions of adjacent cells with the same letter.  Determine the
// perimeter (adjacent cells with different or no letter) and area
// (number of cells in the region).  Multiply perimeter by area to get
// the "price" for the region.  Return the sum of prices of all regions.
//
// This is essentially a connected components problem (where two nodes
// are connected if they contain the same letter).  Solve with a DFS.
// Start with a Hashmap of (row, column) -> letter.
//
pub fn part1(input: &str) -> usize {
    // Parse the input
    let mut plots = IndexMap::new();
    for (row, line) in input.lines().enumerate() {
        for (col, letter) in line.bytes().enumerate() {
            // The +1's below prevent underflow when trying to access
            // neighbors in previous rows or columns.
            plots.insert((row+1, col+1), letter);
        }
    }

    let mut price = 0;

    while let Some(((row, col), letter)) = plots.pop() {
        // eprintln!("{} @ ({row}, {col}):", letter as char);
        let mut connected = HashSet::from([(row, col)]);
        let mut frontier = Vec::from([(row-1, col), (row+1,col), (row,col-1), (row,col+1)]);
        let mut perimeter = 0;

        while let Some((row, col)) = frontier.pop() {
            if connected.contains(&(row, col)) {
                continue;
            }
            if plots.get(&(row, col)) == Some(&letter) {
                // eprintln!("  ({row},{col}) connected");
                plots.swap_remove(&(row, col));
                connected.insert((row, col));
                for (r,c) in [(row-1,col), (row+1,col), (row,col-1), (row,col+1)] {
                    frontier.push((r,c));
                }
            } else {
                // eprintln!("  ({row},{col}) perimeter");
                perimeter += 1;
            }
        }

        // eprintln!("{}: perimeter={}, area={}", letter as char, perimeter, connected.len());

        price += perimeter * connected.len();
    }

    price
}

//
// Like part 1, but instead of perimeter, we use number of sides.
//
pub fn part2(_input: &str) -> usize {
    todo!()
}

#[test]
fn test_part1_ex1() {
    let input = "\
AAAA
BBCD
BBCC
EEEC
";
    assert_eq!(part1(input), 140);
}

#[test]
fn test_part1_ex2() {
    let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    assert_eq!(part1(input), 772);
}

#[test]
fn test_part1_ex3() {
    let input = "\
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
    assert_eq!(part1(input), 1930);
}

#[test]
fn test_part2_ex1() {
    let input = "\
AAAA
BBCD
BBCC
EEEC
";
    assert_eq!(part2(input), 80);
}

#[test]
fn test_part2_ex2() {
    let input = "\
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
    assert_eq!(part2(input), 436);
}

#[test]
fn test_part2_ex3() {
    let input = "\
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
    assert_eq!(part2(input), 236);
}

#[test]
fn test_part2_ex4() {
    let input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
    assert_eq!(part2(input), 368);
}

#[test]
fn test_part2_ex5() {
    let input = "\
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
    assert_eq!(part2(input), 1206);
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 1375476);
}
