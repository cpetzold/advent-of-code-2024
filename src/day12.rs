use std::collections::{HashSet, VecDeque};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use itertools::Itertools;
use nom::{
    character::complete::{line_ending, satisfy},
    multi::many1,
    sequence::terminated,
    IResult,
};
use vecgrid::Vecgrid;

fn end_of_line(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        Ok((input, input))
    } else {
        line_ending(input)
    }
}

#[derive(Clone, Debug)]
pub struct Map {
    grid: Vecgrid<char>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Plot {
    position: IVec2,
    plant: char,
    edges: Vec<IVec2>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Region {
    plant: char,
    plots: HashSet<Plot>,
}

impl Region {
    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        self.plots.iter().map(|plot| plot.edges.len()).sum()
    }

    fn edges_at(&self, position: IVec2, side: IVec2) -> Vec<Vec<i32>> {
        let column = side.x != 0;
        let mut result = Vec::new();
        let mut current_edge = Vec::new();

        for p in self
            .plots
            .iter()
            .filter(|p| {
                ((column && p.position.x == position.x) || (!column && p.position.y == position.y))
                    && p.edges.contains(&side)
            })
            .map(|p| if column { p.position.y } else { p.position.x })
            .sorted_by(|a, b| a.cmp(&b))
        {
            if current_edge.is_empty() || p == current_edge.last().unwrap() + 1 {
                current_edge.push(p);
            } else {
                result.push(current_edge);
                current_edge = vec![p];
            }
        }

        if !current_edge.is_empty() {
            result.push(current_edge);
        }

        result
    }

    fn num_edges_at(&self, position: IVec2, side: IVec2) -> usize {
        self.edges_at(position, side).len()
    }

    fn num_edges(&self) -> usize {
        let mut visited_columns: HashSet<i32> = HashSet::new();
        let mut visited_rows: HashSet<i32> = HashSet::new();
        let mut result = 0;

        for plot in &self.plots {
            if !visited_columns.contains(&plot.position.x) {
                visited_columns.insert(plot.position.x);

                result += self.num_edges_at(plot.position, IVec2::NEG_X)
                    + self.num_edges_at(plot.position, IVec2::X);
            }

            if !visited_rows.contains(&plot.position.y) {
                visited_rows.insert(plot.position.y);

                result += self.num_edges_at(plot.position, IVec2::NEG_Y)
                    + self.num_edges_at(plot.position, IVec2::Y);
            }
        }

        result
    }

    fn fence_cost_perimeter(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn fence_cost_edges(&self) -> usize {
        self.area() * self.num_edges()
    }

    fn has_plot(&self, position: IVec2) -> bool {
        self.plots.iter().any(|p| p.position == position)
    }
}

const ALL_DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

impl Map {
    fn from_str(input: &str) -> Self {
        let rows = many1(terminated(
            many1(satisfy(|c| c.is_alphanumeric())),
            end_of_line,
        ))(input)
        .unwrap()
        .1;

        Self {
            grid: Vecgrid::from_rows(rows).expect("invalid input"),
        }
    }

    fn get_point(&self, position: IVec2) -> Option<char> {
        self.grid
            .get(position.y as usize, position.x as usize)
            .copied()
    }

    fn march_region(&self, start: IVec2) -> Option<Region> {
        let mut queue: VecDeque<IVec2> = VecDeque::new();
        let initial_c = self.get_point(start)?;

        let mut plots: HashSet<Plot> = HashSet::new();

        queue.push_back(start);

        while let Some(p) = queue.pop_front() {
            let Some(c) = self.get_point(p) else {
                continue;
            };

            if c != initial_c || plots.iter().find(|plot| plot.position == p).is_some() {
                continue;
            }

            let edges = ALL_DIRECTIONS
                .into_iter()
                .filter(|d| !self.get_point(p + d).is_some_and(|c| c == initial_c))
                .collect();

            plots.insert(Plot {
                position: p,
                plant: initial_c,
                edges,
            });

            for d in ALL_DIRECTIONS {
                queue.push_back(p + d);
            }
        }

        Some(Region {
            plant: initial_c,
            plots,
        })
    }

    fn regions(&self) -> Vec<Region> {
        let mut regions: Vec<Region> = Vec::new();

        for ((y, x), _) in self.grid.enumerate_column_major() {
            let p = IVec2::new(x as i32, y as i32);
            if regions.iter().any(|r| r.has_plot(p)) {
                continue;
            }

            let region = self.march_region(p);
            if let Some(region) = region {
                regions.push(region);
            }
        }

        regions
    }

    fn fence_cost_perimeter(&self) -> usize {
        self.regions()
            .iter()
            .map(|r| r.fence_cost_perimeter())
            .sum()
    }

    fn fence_cost_edges(&self) -> usize {
        self.regions().iter().map(|r| r.fence_cost_edges()).sum()
    }
}

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> Map {
    Map::from_str(input)
}

#[aoc(day12, part1)]
pub fn solve_part1(map: &Map) -> usize {
    map.fence_cost_perimeter()
}

#[aoc(day12, part2)]
pub fn solve_part2(map: &Map) -> usize {
    map.fence_cost_edges()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cost_perimeter() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let map = Map::from_str(input);
        assert_eq!(map.regions().len(), 11);
        assert_eq!(map.fence_cost_perimeter(), 1930);
    }

    #[test]
    fn test_cost_edges() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        let map = Map::from_str(input);
        let regions = map.regions();
        assert_eq!(regions.len(), 11);
        assert_eq!(map.fence_cost_edges(), 1206);
    }
}
