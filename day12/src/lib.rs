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
pub fn part2(input: &str) -> usize {
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
            }
        }

        // Now that we have the whole region, determine how many sides it has.
        // But how do we calculate number of sides?  My inclination is to look
        // for horizontal and vertical sides separately.
        let mut region = Vec::from_iter(connected.iter().cloned());
        let mut sides = 0;

        // Horizontal sides
        region.sort_unstable();
        let mut last_row = 0;       // There are no cells in row 0.
        let mut last_col = 0;       // ... or column 0.
        let mut top_run = false;
        let mut bottom_run = false;
        for &(row, col) in region.iter() {
            if row != last_row || col != last_col + 1 {
                top_run = false;
                bottom_run = false;
            }
            if !connected.contains(&(row-1,col)) && !top_run {
                top_run = true;
                sides += 1;
            }
            if !connected.contains(&(row+1,col)) && !bottom_run {
                bottom_run = true;
                sides += 1;
            }
            last_row = row;
            last_col = col;
        }

        // Vertical sides
        region.sort_unstable_by_key(|cell| (cell.1, cell.0));
        let mut last_row = 0;       // There are no cells in row 0.
        let mut last_col = 0;       // ... or column 0.
        let mut left_run = false;
        let mut right_run = false;
        for &(row, col) in region.iter() {
            if col != last_col || row != last_row + 1 {
                left_run = false;
                right_run = false;
            }
            if !connected.contains(&(row,col-1)) && !left_run {
                left_run = true;
                sides += 1;
            }
            if !connected.contains(&(row,col+1)) && !right_run {
                right_run = true;
                sides += 1;
            }
            last_row = row;
            last_col = col;
        }

        eprintln!("{}: area={}, sides={}", letter as char, region.len(), sides);

        price += sides * region.len();
    }

    price
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
