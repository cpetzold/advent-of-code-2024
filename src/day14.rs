use core::time;
use std::{fmt::Display, thread};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{i32, line_ending, newline, space1},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
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
struct Robot {
    pos: IVec2,
    vel: IVec2,
}

fn parse_ivec2(input: &str) -> IResult<&str, IVec2> {
    let (input, (x, y)) = separated_pair(i32, tag(","), i32)(input)?;
    Ok((input, IVec2::new(x, y)))
}

fn parse_robot(input: &str) -> IResult<&str, Robot> {
    let (input, (pos, vel)) = separated_pair(
        preceded(tag("p="), parse_ivec2),
        space1,
        preceded(tag("v="), parse_ivec2),
    )(input)?;
    Ok((input, Robot { pos, vel }))
}

fn wrap(x: i32, min: i32, max: i32) -> i32 {
    min + (x - min).rem_euclid(max - min + 1)
}

#[derive(Debug, Clone)]
pub struct Map {
    size: IVec2,
    robots: Vec<Robot>,
}

impl Map {
    fn step(&mut self, n: usize) {
        for _ in 0..n {
            for robot in self.robots.iter_mut() {
                robot.pos.x = wrap(robot.pos.x + robot.vel.x, 0, self.size.x - 1);
                robot.pos.y = wrap(robot.pos.y + robot.vel.y, 0, self.size.y - 1);
            }
        }
    }

    fn num_in_rect(&self, min: IVec2, max: IVec2) -> usize {
        self.robots
            .iter()
            .filter(|r| r.pos.cmpge(min).all() && r.pos.cmplt(max).all())
            .count()
    }

    fn quadrant_counts(&self) -> [usize; 4] {
        let mid = self.size / 2;
        [
            self.num_in_rect(IVec2::new(0, 0), mid),
            self.num_in_rect(IVec2::new(mid.x + 1, 0), IVec2::new(self.size.x, mid.y)),
            self.num_in_rect(mid + IVec2::ONE, self.size),
            self.num_in_rect(IVec2::new(0, mid.y + 1), IVec2::new(mid.x, self.size.y)),
        ]
    }

    fn has_long_vertical_span(&self) -> bool {
        self.robots
            .iter()
            .into_group_map_by(|r| r.pos.x)
            .into_iter()
            .map(|(_, robots)| {
                let mut largest_span = 0;
                let mut current_span = 0;
                let mut last_y = None;
                for y in robots
                    .iter()
                    .sorted_by(|a, b| a.pos.y.cmp(&b.pos.y))
                    .map(|r| r.pos.y)
                {
                    if last_y.is_some() && y == last_y.unwrap() + 1 {
                        current_span += 1;
                    } else {
                        current_span = 1;
                    }
                    last_y = Some(y);
                    largest_span = largest_span.max(current_span);
                }
                largest_span
            })
            .filter(|span| *span > 5)
            .count()
            >= 5
    }

    fn safety_factor(&self) -> usize {
        let counts = self.quadrant_counts();
        counts.iter().product()
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let num = self
                    .robots
                    .iter()
                    .filter(|r| r.pos == IVec2::new(x, y))
                    .count();
                if num > 0 {
                    write!(f, "{}", num)?;
                } else {
                    write!(f, ".")?;
                }
            }
            if y < self.size.y - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> Vec<Robot> {
    separated_list1(newline, parse_robot)(input).unwrap().1
}

#[aoc(day14, part1)]
pub fn solve_part1(machines: &Vec<Robot>) -> usize {
    let mut map = Map {
        size: IVec2::new(101, 103),
        robots: machines.clone(),
    };

    map.step(100);

    map.safety_factor()
}

#[aoc(day14, part2)]
pub fn solve_part2(machines: &Vec<Robot>) -> usize {
    let mut map = Map {
        size: IVec2::new(101, 103),
        robots: machines.clone(),
    };

    let mut i = 0;
    while !map.has_long_vertical_span() {
        map.step(1);
        i += 1;
    }

    println!("{}", map);

    i
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day14_input_generator() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let robots = input_generator(input);
        let mut map = Map {
            size: IVec2::new(11, 7),
            robots,
        };

        assert_eq!(
            map.to_string(),
            "1.12.......
...........
...........
......11.11
1.1........
.........1.
.......1..."
        );

        map.step(100);

        assert_eq!(
            map.to_string(),
            "......2..1.
...........
1..........
.11........
.....1.....
...12......
.1....1...."
        );

        assert_eq!(map.safety_factor(), 12);
    }
}
