use day23::{ part1, part2_orig, part2_incremental, part2_greedy };

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn bench_part1() {
    part1(INPUT);
}

#[divan::bench]
fn bench_part2_orig() {
    part2_orig(INPUT);
}

#[divan::bench]
fn bench_part2_incremental() {
    part2_incremental(INPUT);
}

#[divan::bench]
fn bench_part2_greedy() {
    part2_greedy(INPUT);
}
