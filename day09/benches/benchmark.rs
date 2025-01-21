use day09::{ part1, part2, part2_heaps };

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
fn bench_part2_heaps() {
    part2_heaps(INPUT);
}
