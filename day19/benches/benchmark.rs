use day19::{ part1, part2_memoize, part2_dynamic };

fn main() {
    divan::main();
}

const INPUT: &str = include_str!("../input.txt");

#[divan::bench]
fn bench_part1() {
    part1(INPUT);
}

#[divan::bench]
fn bench_part2_dynamic() {
    part2_dynamic(INPUT);
}

#[divan::bench]
fn bench_part2_memoize() {
    part2_memoize(INPUT);
}
