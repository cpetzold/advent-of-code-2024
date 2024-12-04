use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

type Grid = Vec<Vec<char>>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Grid {
    input.lines().map(|line| line.chars().collect()).collect()
}

#[derive(Debug, EnumIter, Clone, Copy)]
enum Direction {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl From<Direction> for IVec2 {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::N => IVec2::new(0, -1),
            Direction::NE => IVec2::new(1, -1),
            Direction::E => IVec2::new(1, 0),
            Direction::SE => IVec2::new(1, 1),
            Direction::S => IVec2::new(0, 1),
            Direction::SW => IVec2::new(-1, 1),
            Direction::W => IVec2::new(-1, 0),
            Direction::NW => IVec2::new(-1, -1),
        }
    }
}

fn get_pos(grid: &Grid, pos: IVec2) -> Option<char> {
    if pos.y < 0
        || pos.y >= grid.len() as i32
        || pos.x < 0
        || pos.x >= grid[pos.y as usize].len() as i32
    {
        return None;
    }
    grid.get(pos.y as usize)?.get(pos.x as usize).copied()
}

fn matches_in_direction(grid: &Grid, pos: IVec2, dir: Direction, s: &str) -> bool {
    for (i, c) in s.chars().enumerate() {
        if get_pos(grid, pos + (IVec2::from(dir) * i as i32)) != Some(c) {
            return false;
        }
    }
    true
}

#[aoc(day4, part1)]
pub fn solve_part1(grid: &Vec<Vec<char>>) -> u32 {
    let mut count: u32 = 0;
    let test_str = "XMAS";
    for y in 0..grid.len() as i32 {
        for x in 0..grid[y as usize].len() as i32 {
            let pos = IVec2::new(x, y);
            if matches!(get_pos(grid, pos), Some('X')) {
                for dir in Direction::iter() {
                    if matches_in_direction(grid, pos, dir, test_str) {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

#[aoc(day4, part2)]
pub fn solve_part2(grid: &Vec<Vec<char>>) -> u32 {
    let test_str = "MAS";
    let mut count: u32 = 0;
    for y in 0..grid.len() as i32 {
        for x in 0..grid[y as usize].len() as i32 {
            let pos = IVec2::new(x, y);
            if matches!(get_pos(grid, pos), Some('A'))
                && (matches_in_direction(
                    grid,
                    pos + IVec2::from(Direction::NW),
                    Direction::SE,
                    test_str,
                ) || matches_in_direction(
                    grid,
                    pos + IVec2::from(Direction::SE),
                    Direction::NW,
                    test_str,
                ))
                && (matches_in_direction(
                    grid,
                    pos + IVec2::from(Direction::NE),
                    Direction::SW,
                    test_str,
                ) || matches_in_direction(
                    grid,
                    pos + IVec2::from(Direction::SW),
                    Direction::NE,
                    test_str,
                ))
            {
                count += 1;
            }
        }
    }
    count
}
