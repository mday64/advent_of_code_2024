use pathfinding::prelude::astar;
use rustc_hash::FxHashSet;

//
// Another pathfinding problem
//
pub fn path_length(input: &str, num_points: usize, max_dimension: i32) -> Option<u32> {
    let mut walls = FxHashSet::default();
    for line in input.lines().take(num_points) {
        let (x, y) = line.split_once(",").expect("valid input");
        let x = x.parse::<i32>().expect("valid number");
        let y = y.parse::<i32>().expect("valid number");
        walls.insert((x, y));
    }
    // For simplicity, let's add some walls outside the valid space
    for x in -1 ..= max_dimension+1 {
        walls.insert((x, -1));
        walls.insert((x, max_dimension+1));
    }
    for y in 0 ..= max_dimension {
        walls.insert((-1, y));
        walls.insert((max_dimension+1, y));
    }

    let success = |&(x, y):&(i32,i32)| x==max_dimension && y==max_dimension;
    let heuristic = |&(x,y):&(i32,i32)| max_dimension.abs_diff(x) + max_dimension.abs_diff(y);
    let successors = |&(x,y):&(i32,i32)| {
        let walls = &walls;
        [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter().filter_map(move |(dx,dy)| {
            let xx = x + dx;
            let yy = y + dy;
            if walls.contains(&(xx,yy)) {
                None
            } else {
                Some(((xx, yy), 1))
            }
        })
    };
    let (_path, cost) = astar(&(0,0), successors, heuristic, success)?;
    Some(cost)
}

fn part1(input: &str, num_points: usize, max_dimension: i32) -> u32 {
    path_length(input, num_points, max_dimension).expect("a solution")
}

//
// This time, we have to report the coordinates of the first point
// that makes it impossible to find a path.
//
// We could just keep trying them sequentially, or we could binary search
// to find the critical point.
//
pub fn part2(_input: &str) -> String {
    "World".to_string()
}

#[test]
fn test_part1() {
    let input = "\
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";
    assert_eq!(part1(input, 12, 6), 22);
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
    assert_eq!(part1(FULL_INPUT, 1024, 70), 250);
}