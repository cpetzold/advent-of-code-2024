use aoc_runner_derive::{aoc, aoc_generator};
use glam::I64Vec2;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending, newline},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

fn end_of_line(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        Ok((input, input))
    } else {
        line_ending(input)
    }
}

#[derive(Debug, Clone, Copy)]
struct Machine {
    a: I64Vec2,
    b: I64Vec2,
    prize: I64Vec2,
}

impl Machine {
    fn cheapest_solution_cost(&self) -> Option<i64> {
        let ax = self.a.x;
        let ay = self.a.y;
        let bx = self.b.x;
        let by = self.b.y;
        let px = self.prize.x;
        let py = self.prize.y;

        let b = (py * ax - px * ay) / (by * ax - bx * ay);
        let a = (px - bx * b) / ax;
        if ax * a + bx * b == px && ay * a + by * b == py {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (a, b, prize)) = tuple((
        delimited(
            tag("Button A: "),
            separated_pair(preceded(tag("X+"), i64), tag(", Y+"), i64),
            end_of_line,
        ),
        delimited(
            tag("Button B: "),
            separated_pair(preceded(tag("X+"), i64), tag(", Y+"), i64),
            end_of_line,
        ),
        delimited(
            tag("Prize: "),
            separated_pair(preceded(tag("X="), i64), tag(", Y="), i64),
            end_of_line,
        ),
    ))(input)?;

    Ok((
        input,
        Machine {
            a: a.into(),
            b: b.into(),
            prize: prize.into(),
        },
    ))
}

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Vec<Machine> {
    separated_list1(newline, parse_machine)(input).unwrap().1
}

#[aoc(day13, part1)]
pub fn solve_part1(machines: &Vec<Machine>) -> i64 {
    machines
        .iter()
        .map(|m| m.cheapest_solution_cost().unwrap_or(0))
        .sum()
}

#[aoc(day13, part2)]
pub fn solve_part2(machines: &Vec<Machine>) -> i64 {
    let machines = machines.clone();
    machines
        .iter()
        .map(|m| {
            let mut m = m.clone();
            m.prize += I64Vec2::new(10000000000000, 10000000000000);
            m.cheapest_solution_cost().unwrap_or(0)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn test_day13_input_generator() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";
        let machines = input_generator(input);
        dbg!(&machines);
        dbg!(machines
            .iter()
            .map(|m| m.cheapest_solution_cost())
            .collect_vec());
    }
}
