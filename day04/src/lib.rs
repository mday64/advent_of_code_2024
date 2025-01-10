//
// Count the number of ways "XMAS" appears in the input.  It could be
// forwards or backwards, up or down, or diagonal.
//
// Note: the input consists of only the letters X, M, A, S, and newlines.
// I'm not sure if that's helpful.
//
// Note: the input is a square grid (same number of rows and columns, if
// you ignore the newlines).
//
#[allow(clippy::needless_range_loop)]
pub fn part1(input: &str) -> u32 {
    let mut grid: Vec<Vec<char>> = vec![];
    for line in input.lines() {
        grid.push(line.chars().collect());
    }
    assert_eq!(grid.len(), grid[0].len());
    let dimension = grid.len();

    let forwards = ['X', 'M', 'A', 'S'];
    let backwards = ['S', 'A', 'M', 'X'];

    let mut result = 0;
    let mut window = [' ', ' ', ' ', ' '];
    
    // Look for horizontal words
    for row in 0..dimension {
        window.fill(' ');
        for col in 0..dimension {
            let ch = grid[row][col];
            window.rotate_left(1);
            window[3] = ch;
            if window == forwards || window == backwards {
                eprintln!("Horizontal (starting row={} col={}", row, col-3);
                result += 1;
            }
        }
    }

    // Look for vertical words
    for col in 0..dimension {
        window.fill(' ');
        for row in 0..dimension {
            let ch = grid[row][col];
            window.rotate_left(1);
            window[3] = ch;
            if window == forwards || window == backwards {
                eprintln!("Vertical (ending row={} col={}", row-3, col);
                result += 1;
            }
        }
    }

    // Look for diagonal words (upper left to lower right)
    // This is not terribly efficient.  It might be better to follow
    // a diagonal from edge to edge, rotating `window` like above.
    // But that's a bit trickier to get right.
    for col in 0..dimension-3 {
        for row in 0..(dimension-3) {
            // We are now looking at 4 characters starting at grid[row][col]
            for i in 0..=3 {
                window[i] = grid[row+i][col+i];
            }
            if window == forwards || window == backwards {
                eprintln!("Diagonal1 (starting row={} col={}", row, col);
                result += 1;
            }
        }
    }

    // Look for diagonal words (upper right to lower left)
    for col in 3..dimension {
        for row in 0..dimension-3 {
            // We are now looking at 4 characters starting at grid[row][col]
            for i in 0..=3 {
                window[i] = grid[row+i][col-i];
            }
            if window == forwards || window == backwards {
                eprintln!("Diagonal2 (starting row={} col={}", row, col);
                result += 1;
            }
        }
    }
    
    result
}

#[test]
fn test_part1() {
    let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    assert_eq!(part1(input), 18);
}
