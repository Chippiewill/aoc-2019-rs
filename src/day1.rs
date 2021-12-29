use std::cmp;

use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u64> {
	input.lines().map(|l| l.parse::<u64>().unwrap()).collect_vec()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u64]) -> u64 {
    input.iter().map(|v| (v / 3) - 2).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u64]) -> u64 {
    fn solver(v: u64) -> u64 {
	    if v == 0 {
	        return 0;
	    }

	    let fuel = (v / 3) - cmp::min(v / 3, 2);
        fuel + solver(fuel)
	}

    input.iter().cloned().map(solver).sum()
}
