use std::collections::{BTreeSet, HashMap};

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    character::complete::{char, newline, u8},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
};

#[derive(Debug)]
pub struct SafetyManual {
    ordering_rules: BTreeSet<(u8, u8)>,
    updates: Vec<Vec<u8>>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> SafetyManual {
    let (ordering_rules, updates) = separated_pair(
        separated_list1(
            newline::<&str, nom::error::Error<&str>>,
            separated_pair(u8, char('|'), u8),
        ),
        tuple((newline, newline)),
        separated_list1(newline, separated_list1(char(','), u8)),
    )(input)
    .unwrap()
    .1;

    SafetyManual {
        ordering_rules: BTreeSet::from_iter(ordering_rules.into_iter()),
        updates,
    }
}

#[aoc(day5, part1)]
pub fn solve_part1(manual: &SafetyManual) -> u32 {
    manual
        .updates
        .iter()
        .filter(|update| {
            update
                .iter()
                .tuple_combinations()
                .all(|(&x, &y)| manual.ordering_rules.contains(&(x, y)))
        })
        .map(|update| update[update.len() / 2] as u32)
        .sum()
}

#[aoc(day5, part2)]
pub fn solve_part2(manual: &SafetyManual) -> u32 {
    manual
        .updates
        .iter()
        .filter(|update| {
            !update
                .iter()
                .tuple_combinations()
                .all(|(&x, &y)| manual.ordering_rules.contains(&(x, y)))
        })
        .map(|update| {
            let mut counts: HashMap<u8, i32> = HashMap::new();
            for (&x, &y) in update
                .iter()
                .permutations(2)
                .map(|x| x.into_iter().collect_tuple().unwrap())
            {
                if manual.ordering_rules.contains(&(x, y)) {
                    *counts.entry(x).or_insert(0) -= 1;
                    *counts.entry(y).or_insert(0) += 1;
                }
            }

            counts
                .iter()
                .find(|(&_, &count)| count == 0)
                .unwrap()
                .0
                .clone() as u32
        })
        .sum()
}
