use pathfinding::prelude::astar;
use rustc_hash::FxHashSet;

struct Point {
    x: i32,
    y: i32
}

fn parse_input(input: &str) -> Vec<Point> {
    input.lines().map(|line| {
        let (x,y) = line.split_once(",").expect("a point");
        let x = x.parse::<i32>().expect("valid number");
        let y = y.parse::<i32>().expect("valid number");
        Point{x,y}
    }).collect()
}

//
// Another pathfinding problem
//
fn path_length(points: &[Point], num_points: usize, max_dimension: i32) -> Option<u32> {
    let mut walls = FxHashSet::default();
    for &Point{x,y} in points.iter().take(num_points) {
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

pub fn part1(input: &str, num_points: usize, max_dimension: i32) -> u32 {
    let points = parse_input(input);
    path_length(&points, num_points, max_dimension).expect("a solution")
}

//
// This time, we have to report the coordinates of the first point
// that makes it impossible to find a path.
//
// We could just keep trying them sequentially, or we could binary search
// to find the critical point.
//
pub fn part2(input: &str, num_points: usize, max_dimension: i32) -> (i32, i32) {
    let points = parse_input(input);

    let mut solvable = num_points;        // From part 1, we know this many points is OK
    let mut unsolvable = input.lines().count();

    while unsolvable - solvable > 1 {
        let middle = (unsolvable + solvable) / 2;
        if path_length(&points, middle, max_dimension).is_some() {
            solvable = middle;
        } else {
            unsolvable = middle;
        }
    }
    
    let Point{x,y} = points[unsolvable-1];
    (x,y)
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
    assert_eq!(part2(input, 12, 6), (6,1));
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT, 1024, 70), 250);
}

#[test]
fn test_part2_full() {
    assert_eq!(part2(FULL_INPUT, 1024, 70), (56, 8));
}
