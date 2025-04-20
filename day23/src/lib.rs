use std::collections::{HashMap, HashSet};
use rustc_hash::FxHashSet;

pub use part2_greedy as part2;

pub fn part1(input: &str) -> usize {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').expect("hyphen");
        // Insert connections such that the strings point to "larger" strings
        if left < right {
            connections.entry(left).or_default().insert(right);
        } else {
            connections.entry(right).or_default().insert(left);
        }
    }

    let mut result = 0;
    for (first, seconds) in &connections {
        for second in seconds {
            if let Some(thirds) = connections.get(second) {
                for third in thirds {
                    if (first.starts_with("t") || second.starts_with("t") || third.starts_with("t"))
                        && seconds.contains(third)
                    {
                        // println!("{first},{second},{third}");
                        result += 1;
                    }
                }
            }
        }
    }
    result
}

//
// Find the largest strongly connected component of the graph.
//
// From the problem description, we know that there is a unique largest
// strongly connected component.
//
// My approach is going to iteratively find all strongly connected components
// of size N, N+1, N+2, ..., until there is only one connected component.
// The input is, by definition, all strongly connected components of size 2.
//
// I will store a strongly connected component as a Vec<&str>, with the
// strings in sorted order.
//
pub fn part2_orig(input: &str) -> String {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();
    let mut connected_components: Vec<Vec<&str>> = Vec::new();
    let mut nodes = HashSet::new();
    for line in input.lines() {
        let (left, right) = line.split_once('-').expect("hyphen");
        nodes.insert(left);
        nodes.insert(right);
        // Insert connections such that the strings point to "larger" strings
        if left < right {
            connections.entry(left).or_default().insert(right);
            connected_components.push(vec![left, right]);
        } else {
            connections.entry(right).or_default().insert(left);
            connected_components.push(vec![right, left]);
        }
    }

    // Helper closure to test whether two nodes are connected (in sorted order)
    let is_connected = |a: &&str, b: &&str| connections.get(a).is_some_and(|set| set.contains(b));

    while connected_components.len() > 1 {
        let mut larger = Vec::new();    // connected components of size N+1
        for component in connected_components {
            for node in &nodes {
                // If every node in `component` is connected to `node`, then
                // we can merge `node` into `component` to make a larger one.
                if component.iter().all(|src| is_connected(src, node)) {
                    let mut temp = component.clone();
                    temp.push(node);
                    larger.push(temp);
                }
            }
        }
        connected_components = larger;
    }

    assert_eq!(connected_components.len(), 1);
    connected_components[0].join(",")
}

//
// Same basic approach as part2_orig, but stores connections differently.
//
pub fn part2_incremental(input: &str) -> String {
    let mut connections: FxHashSet<(&str, &str)> = FxHashSet::default();
    let mut connected_components: Vec<Vec<&str>> = Vec::new();
    let mut nodes = FxHashSet::default();
    for line in input.lines() {
        let (left, right) = line.split_once('-').expect("hyphen");
        nodes.insert(left);
        nodes.insert(right);
        // Insert connections such that the strings point to "larger" strings
        if left < right {
            connections.insert((left, right));
            connected_components.push(vec![left, right]);
        } else {
            connections.insert((right, left));
            connected_components.push(vec![right, left]);
        }
    }

    while connected_components.len() > 1 {
        let mut larger = Vec::new();    // connected components of size N+1
        for component in connected_components {
            for node in &nodes {
                // If every node in `component` is connected to `node`, then
                // we can merge `node` into `component` to make a larger one.
                if component.iter().all(|src| connections.contains(&(src,node))) {
                    let mut temp = component.clone();
                    temp.push(node);
                    larger.push(temp);
                }
            }
        }
        connected_components = larger;
    }

    assert_eq!(connected_components.len(), 1);
    connected_components[0].join(",")
}

pub fn part2_greedy(input: &str) -> String {
    let mut connections: FxHashSet<(&str, &str)> = FxHashSet::default();
    let mut nodes: FxHashSet<&str> = FxHashSet::default();

    for line in input.lines() {
        let (left, right) = line.split_once('-').expect("hyphen");
        nodes.insert(left);
        nodes.insert(right);
        if left < right {
            connections.insert((left, right));
        } else {
            connections.insert((right, left));
        }
    }
    let mut nodes: Vec<&str> = nodes.into_iter().collect();
    nodes.sort();

    let mut components: Vec<Vec<&str>> = Vec::new();
    for node in &nodes {
        let mut merged: Vec<Vec<&str>> = Vec::new();
        for component in &components {
            if component.iter().all(|&other| connections.contains(&(other, node))) {
                let mut temp = component.clone();
                temp.push(node);
                merged.push(temp);
            }
        }
        components.append(&mut merged);
        components.push(vec![node]);
    }

    let largest = components.iter().max_by_key(|component| component.len()).unwrap();
    largest.join(",")
}

#[test]
fn test_part1() {
    let input = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    assert_eq!(part1(input), 7);
}

#[test]
fn test_part2_orig() {
    let input = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    assert_eq!(part2_orig(input), "co,de,ka,ta");
}

#[test]
fn test_part2_incremental() {
    let input = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    assert_eq!(part2_incremental(input), "co,de,ka,ta");
}

#[test]
fn test_part2_greedy() {
    let input = "\
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";
    assert_eq!(part2_greedy(input), "co,de,ka,ta");
}

#[cfg(test)]
static FULL_INPUT: &str = include_str!("../input.txt");

#[test]
fn test_part1_full() {
    assert_eq!(part1(FULL_INPUT), 1370);
}

#[test]
fn test_part2_orig_full() {
    assert_eq!(part2_orig(FULL_INPUT), "am,au,be,cm,fo,ha,hh,im,nt,os,qz,rr,so");
}

#[test]
fn test_part2_incremental_full() {
    assert_eq!(part2_incremental(FULL_INPUT), "am,au,be,cm,fo,ha,hh,im,nt,os,qz,rr,so");
}

#[test]
fn test_part2_greedy_full() {
    assert_eq!(part2_greedy(FULL_INPUT), "am,au,be,cm,fo,ha,hh,im,nt,os,qz,rr,so");
}
