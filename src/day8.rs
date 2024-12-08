use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use itertools::Itertools;
use nom::{
    character::{
        complete::{line_ending, satisfy},
        is_alphanumeric,
    },
    multi::many1,
    sequence::terminated,
    IResult,
};

#[derive(Debug, Clone)]
pub struct Map {
    size: IVec2,
    antennas: HashMap<char, Vec<IVec2>>,
    pub resonant_harmonics: bool,
}

impl Map {
    fn get_antenna(&self, position: IVec2) -> Option<char> {
        for (c, antennas) in &self.antennas {
            if antennas.contains(&position) {
                return Some(*c);
            }
        }

        None
    }

    fn in_bounds(&self, position: &IVec2) -> bool {
        position.x >= 0 && position.x < self.size.x && position.y >= 0 && position.y < self.size.y
    }

    fn antenna_pair_antinodes(&self, a: &IVec2, b: &IVec2) -> HashSet<IVec2> {
        let mut antinodes = HashSet::new();

        if self.resonant_harmonics {
            let d = b - a;
            let mut curr = a.clone();
            while self.in_bounds(&curr) {
                antinodes.insert(curr);
                curr -= d;
            }

            let d = a - b;
            curr = b.clone();
            while self.in_bounds(&curr) {
                antinodes.insert(curr);
                curr -= d;
            }
        } else {
            let x = a - (b - a);
            let y = b - (a - b);
            if self.in_bounds(&x) {
                antinodes.insert(x);
            }
            if self.in_bounds(&y) {
                antinodes.insert(y);
            }
        }

        antinodes
    }

    fn get_antinodes(&self) -> HashSet<IVec2> {
        self.antennas
            .values()
            .fold(HashSet::new(), |mut acc, antennas| {
                for (a, b) in antennas.iter().tuple_combinations() {
                    acc.extend(&self.antenna_pair_antinodes(a, b));
                }

                acc
            })
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let antinodes = self.get_antinodes();
        for y in 0..self.size.y {
            for x in 0..self.size.x {
                let pos = IVec2::new(x, y);
                let antenna = self.get_antenna(pos);
                if let Some(c) = antenna {
                    write!(f, "{}", c)?;
                } else if antinodes.contains(&pos) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            if y != self.size.y - 1 {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

fn end_of_line(input: &str) -> IResult<&str, &str> {
    if input.is_empty() {
        Ok((input, input))
    } else {
        line_ending(input)
    }
}

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Map {
    let raw_map = many1(terminated(
        many1(satisfy(|c| c == '.' || is_alphanumeric(c as u8))),
        end_of_line,
    ))(input)
    .unwrap()
    .1;

    let height = raw_map.len();
    let width = raw_map[0].len();

    let mut antennas: HashMap<char, Vec<IVec2>> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let c = raw_map[y][x];
            if c != '.' {
                antennas
                    .entry(c)
                    .or_default()
                    .push(IVec2::new(x as i32, y as i32));
            }
        }
    }

    Map {
        size: IVec2::new(width as i32, height as i32),
        antennas,
        resonant_harmonics: false,
    }
}

#[aoc(day8, part1)]
pub fn solve_part1(map: &Map) -> usize {
    let antinodes = map.get_antinodes();

    println!("{}", map);

    antinodes.len()
}

#[aoc(day8, part2)]
pub fn solve_part2(map: &Map) -> usize {
    let mut map = map.clone();
    map.resonant_harmonics = true;
    solve_part1(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day8_part1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let map = input_generator(&input);

        let output = format!("{}", map);

        assert_eq!(
            output,
            "......#....#
...#....0...
....#0....#.
..#....0....
....0....#..
.#....A.....
...#........
#......#....
........A...
.........A..
..........#.
..........#."
        );

        assert_eq!(solve_part1(&map), 9);
    }

    #[test]
    fn test_day8_part2() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

        let map = input_generator(&input);

        let output = solve_part2(&map);

        assert_eq!(output, 34);
    }
}
