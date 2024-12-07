use std::iter::once;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{repeat_n, Itertools};
use nom::{
    bytes::complete::tag,
    character::complete::{i64, newline},
    multi::separated_list1,
    sequence::separated_pair,
};

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<(i64, Vec<i64>)> {
    separated_list1(
        newline::<&str, nom::error::Error<&str>>,
        separated_pair(i64, tag(": "), separated_list1(tag(" "), i64)),
    )(input)
    .unwrap()
    .1
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Multiply,
    Concat,
}

fn get_operators(
    solution: i64,
    operands: &Vec<i64>,
    valid_operators: Vec<Operator>,
) -> Option<Vec<Operator>> {
    let num_operators = operands.len() - 1;
    let possible_arrangements = repeat_n(valid_operators, num_operators).multi_cartesian_product();

    possible_arrangements.into_iter().find(|operators| {
        let result = operands
            .iter()
            .zip(once(&Operator::Add).chain(operators.into_iter()))
            .fold(0, |acc, (operand, operator)| match operator {
                Operator::Add => acc + operand,
                Operator::Multiply => acc * operand,
                Operator::Concat => format!("{}{}", acc, operand).parse().unwrap(),
            });

        result == solution
    })
}

#[aoc(day7, part1)]
pub fn solve_part1(formulas: &Vec<(i64, Vec<i64>)>) -> i64 {
    let valid_formulas = formulas.iter().filter(|(solution, operands)| {
        get_operators(*solution, operands, vec![Operator::Add, Operator::Multiply]).is_some()
    });

    valid_formulas.map(|(solution, _)| solution).sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(formulas: &Vec<(i64, Vec<i64>)>) -> i64 {
    let valid_formulas = formulas.iter().filter(|(solution, operands)| {
        get_operators(
            *solution,
            operands,
            vec![Operator::Add, Operator::Multiply, Operator::Concat],
        )
        .is_some()
    });

    valid_formulas.map(|(solution, _)| solution).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let map = input_generator(input);
        dbg!(solve_part1(&map));

        let operators = get_operators(3, &vec![1, 1, 2]);
        dbg!(operators);
    }
}
