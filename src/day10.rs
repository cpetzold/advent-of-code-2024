use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use itertools::Itertools;
use nom::{
    character::complete::{line_ending, satisfy},
    multi::many1,
    sequence::terminated,
    IResult, Parser,
};
use vecgrid::Vecgrid;

fn end_of_line(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        Ok((input, input))
    } else {
        line_ending(input)
    }
}

type Trail = Vec<IVec2>;

#[derive(Debug)]
struct Map {
    grid: Vecgrid<u8>,
}

impl Map {
    fn get_point(&self, position: IVec2) -> Option<u8> {
        self.grid
            .get(position.y as usize, position.x as usize)
            .copied()
    }

    fn trails_from(&self, position: IVec2, previous: &Trail) -> Vec<Trail> {
        match self.get_point(position) {
            None => vec![previous.clone()],
            Some(current) => {
                let mut previous = previous.clone();
                previous.push(position);

                if current == 9 {
                    return vec![previous];
                } else {
                    [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
                        .into_iter()
                        .filter(|direction| {
                            self.get_point(position + direction)
                                .is_some_and(|point| point == current + 1)
                        })
                        .flat_map(|direction| self.trails_from(position + direction, &previous))
                        .filter(|trail| {
                            trail
                                .last()
                                .is_some_and(|last| matches!(self.get_point(*last), Some(9)))
                        })
                        .collect()
                }
            }
        }
    }

    fn trails(&self) -> Vec<Trail> {
        self.grid
            .enumerate_column_major()
            .filter_map(|(position, point)| {
                if *point == 0 {
                    Some(
                        self.trails_from(IVec2::new(position.1 as i32, position.0 as i32), &vec![]),
                    )
                } else {
                    None
                }
            })
            .flatten()
            .collect()
    }
}

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> Map {
    let rows = many1(terminated(
        many1(satisfy(|c| c.is_digit(10)).map(|c| c.to_digit(10).unwrap() as u8)),
        end_of_line,
    ))(input)
    .unwrap()
    .1;

    Map {
        grid: Vecgrid::from_rows(rows).expect("invalid input"),
    }
}

#[aoc(day10, part1)]
pub fn solve_part1(map: &Map) -> usize {
    map.trails()
        .into_iter()
        .unique_by(|trail| (trail.first().cloned(), trail.last().cloned()))
        .chunk_by(|trail| trail.first().cloned())
        .into_iter()
        .map(|(_, trails)| trails.count())
        .sum()
}

#[aoc(day10, part2)]
pub fn solve_part2(map: &Map) -> usize {
    map.trails()
        .into_iter()
        .chunk_by(|trail| trail.first().cloned())
        .into_iter()
        .map(|(_, trails)| trails.count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day10_part1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let map = input_generator(&input);

        assert_eq!(solve_part1(&map), 36);
    }

    #[test]
    fn test_day10_part2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

        let map = input_generator(&input);

        assert_eq!(solve_part2(&map), 81);
    }
}
