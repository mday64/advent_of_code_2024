use day13::{ part1, part2, parse_machines };

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn bench_part1() {
    part1(INPUT);
}

#[divan::bench]
fn bench_part2() {
    part2(INPUT);
}

#[divan::bench]
fn bench_parsing() {
    let (_, machines) = parse_machines(INPUT).expect("well formed input");
    assert!(machines.len() > 1);
}
