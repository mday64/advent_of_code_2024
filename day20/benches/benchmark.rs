use day20::{ part1, part2_limit_v1, part2_limit_v2 };

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn bench_part1() {
    part1(INPUT);
}

#[divan::bench]
fn bench_part2_limit() {
    part2_limit_v1(INPUT, 100);
}

#[divan::bench]
fn bench_part2_v2() {
    part2_limit_v2(INPUT, 100);
}
