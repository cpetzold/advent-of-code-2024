use aoc_runner_derive::{aoc, aoc_generator};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, u32},
    combinator::map,
    multi::{many0, many_till},
    sequence::{delimited, separated_pair},
    Parser,
};

fn parse_mul(input: &str) -> nom::IResult<&str, (u32, u32)> {
    delimited(tag("mul("), separated_pair(u32, tag(","), u32), tag(")"))(input)
}

fn parse_all_mul(input: &str) -> nom::IResult<&str, Vec<(u32, u32)>> {
    many0(many_till(anychar, parse_mul).map(|(_, mul)| mul))(input)
}

#[aoc_generator(day3, part1)]
pub fn input_generator_part1(input: &str) -> Vec<(u32, u32)> {
    parse_all_mul(input).unwrap().1
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<(u32, u32)>) -> u32 {
    input.iter().map(|(x, y)| x * y).sum()
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_all_mul_with_conds(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    many0(
        many_till(
            anychar,
            alt((
                map(parse_mul, |mul| Instruction::Mul(mul.0, mul.1)),
                map(tag("do()"), |_| Instruction::Do),
                map(tag("don't()"), |_| Instruction::Dont),
            )),
        )
        .map(|(_, i)| i),
    )(input)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &str) -> u32 {
    let (_, insts) = parse_all_mul_with_conds(input).unwrap();

    let mut sum = 0;
    let mut do_mul = true;
    for inst in insts {
        match inst {
            Instruction::Mul(x, y) => {
                if do_mul {
                    sum += x * y;
                }
            }
            Instruction::Do => do_mul = true,
            Instruction::Dont => do_mul = false,
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_mul() {
        assert_eq!(parse_mul("mul(1,2)"), Ok(("", (1, 2))));
        assert_eq!(parse_mul("mul(1,2)abc"), Ok(("abc", (1, 2))));
        assert_eq!(
            parse_mul("abc"),
            Err(nom::Err::Error(nom::error::Error::new(
                "abc",
                nom::error::ErrorKind::Tag
            )))
        );
    }

    #[test]
    fn test_parse_all_mul() {
        assert_eq!(parse_all_mul("mul(1,2)"), Ok(("", vec![(1, 2)])));
        assert_eq!(
            parse_all_mul("mul(1,2)mul(3,4)"),
            Ok(("", vec![(1, 2), (3, 4)]))
        );
        assert_eq!(parse_all_mul("mul(1,2)abc"), Ok(("abc", vec![(1, 2)])));
        assert_eq!(parse_all_mul("mfoomul(1,2)bar"), Ok(("bar", vec![(1, 2)])));
        assert_eq!(
            parse_all_mul("foomul(1,2)barmul(3,4)xyz"),
            Ok(("xyz", vec![(1, 2), (3, 4)]))
        );
        assert_eq!(
            parse_all_mul(
                "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
            ),
            Ok((")", vec![(2, 4), (5, 5), (11, 8), (8, 5)]))
        );
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(solve_part2("don't()"), 0);
    }

    #[test]
    fn test_parse_all_mul_with_conds() {
        assert_eq!(parse_all_mul_with_conds(""), Ok(("", vec![])));
        assert_eq!(
            parse_all_mul_with_conds("mul(1,2)don't()do()"),
            Ok((
                "",
                vec![Instruction::Mul(1, 2), Instruction::Dont, Instruction::Do]
            ))
        );
        assert_eq!(
            parse_all_mul_with_conds("mul(1,2)don't()do()mul(3,4)"),
            Ok((
                "",
                vec![
                    Instruction::Mul(1, 2),
                    Instruction::Dont,
                    Instruction::Do,
                    Instruction::Mul(3, 4)
                ]
            ))
        );
    }
}
