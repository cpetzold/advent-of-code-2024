use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    character::complete::{multispace1, newline, u32},
    multi::separated_list0,
    sequence::separated_pair,
};
use std::collections::BTreeMap;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<u32>, Vec<u32>) {
    separated_list0(
        newline::<&str, nom::error::Error<&str>>,
        separated_pair(u32, multispace1, u32),
    )(input)
    .map(|(_, result)| result.iter().cloned().collect())
    .unwrap()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &(Vec<u32>, Vec<u32>)) -> u32 {
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
pub fn solve_part2(input: &(Vec<u32>, Vec<u32>)) -> u32 {
    let mut occurances: BTreeMap<u32, u32> = BTreeMap::new();
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
