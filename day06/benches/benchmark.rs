use day06::{ part1, part2, both_parts, both_parts_cached };

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
fn bench_both_parts() {
    both_parts(INPUT);
}

#[divan::bench]
#[allow(non_snake_case)]
fn bench_both_parts_WIP() {
    both_parts_cached(INPUT);
}
