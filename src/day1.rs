use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .lines()
        .map(|l| {
            let (left, right) = l
                .split_once(' ')
                .map(|(l, r)| {
                    (
                        l.trim().parse::<usize>().unwrap(),
                        r.trim().parse::<usize>().unwrap(),
                    )
                })
                .unwrap();
            (left, right)
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut left = input.0.clone();
    left.sort();
    let mut right = input.1.clone();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| l.abs_diff(*r))
        .sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let mut occurances: BTreeMap<usize, usize> = BTreeMap::new();
    for x in input.1.iter() {
        occurances
            .entry(*x)
            .and_modify(|curr| *curr += 1)
            .or_insert(1);
    }

    input
        .0
        .iter()
        .map(|x| x * occurances.get(x).unwrap_or(&0))
        .sum()
}
