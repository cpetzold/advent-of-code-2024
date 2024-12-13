use std::collections::BTreeMap;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use nom::{
    character::complete::{line_ending, space1, u64},
    multi::separated_list1,
    sequence::terminated,
    IResult,
};

#[derive(Clone)]
pub struct Stones {
    stones: BTreeMap<u64, usize>,
}

impl Stones {
    fn from_str(input: &str) -> Self {
        let stones = terminated(separated_list1(space1, u64), end_of_line)(input)
            .unwrap()
            .1;
        Self {
            stones: stones.into_iter().counts().into_iter().collect(),
        }
    }
}

impl Iterator for Stones {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let count = self.stones.values().sum();

        let mut stones = BTreeMap::new();

        for (stone, count) in self.stones.iter() {
            match stone {
                0 => *stones.entry(1).or_insert(0) += count,
                n => {
                    let num_digits = n.checked_ilog10().unwrap_or(0) + 1;
                    if num_digits % 2 == 0 {
                        let n_str = n.to_string();
                        let (left, right) = n_str.split_at(num_digits as usize / 2);
                        let left = left.parse::<u64>().unwrap();
                        let right = right.parse::<u64>().unwrap();
                        *stones.entry(left).or_insert(0) += count;
                        *stones.entry(right).or_insert(0) += count;
                    } else {
                        let new_stone = n * 2024;
                        *stones.entry(new_stone).or_insert(0) += count;
                    }
                }
            }
        }

        self.stones = stones;

        Some(count)
    }
}

fn end_of_line(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        Ok((input, input))
    } else {
        line_ending(input)
    }
}

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> Stones {
    Stones::from_str(input)
}

#[aoc(day11, part1)]
pub fn solve_part1(stones: &Stones) -> usize {
    let mut stones = stones.clone();
    stones.nth(25).unwrap()
}

#[aoc(day11, part2)]
pub fn solve_part2(stones: &Stones) -> usize {
    let mut stones = stones.clone();
    stones.nth(75).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_stones() {
        let input = "125 17";
        let mut stones = Stones::from_str(input);
        assert_eq!(stones.next(), Some(2));
        assert_eq!(stones.next(), Some(3));
        assert_eq!(stones.next(), Some(4));
        assert_eq!(stones.nth(22), Some(55312));
    }
}
