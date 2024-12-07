use std::collections::{HashMap, HashSet};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
};

#[derive(Debug)]
pub struct Map {
    size: IVec2,
    obstructions: HashSet<IVec2>,
    guard: IVec2,
}

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Map {
    let raw_map = separated_list1(
        newline::<&str, nom::error::Error<&str>>,
        many1(one_of("^#.")),
    )(input)
    .unwrap()
    .1;

    let height = raw_map.len();
    let width = raw_map[0].len();

    let mut obstructions = HashSet::new();
    let mut guard = IVec2::ZERO;

    for y in 0..height {
        for x in 0..width {
            if raw_map[y][x] == '#' {
                obstructions.insert(IVec2::new(x as i32, y as i32));
            }
            if raw_map[y][x] == '^' {
                guard = IVec2::new(x as i32, y as i32);
            }
        }
    }

    Map {
        size: IVec2::new(width as i32, height as i32),
        obstructions,
        guard,
    }
}

fn in_map(map: &Map, position: IVec2) -> bool {
    position.x >= 0 && position.x < map.size.x && position.y >= 0 && position.y < map.size.y
}

fn get_visited(map: &Map) -> Option<HashMap<IVec2, HashSet<IVec2>>> {
    let mut position = map.guard.clone();
    let mut direction = IVec2::NEG_Y;
    let mut visited: HashMap<IVec2, HashSet<IVec2>> = HashMap::new();

    while in_map(map, position) {
        if visited
            .get(&position)
            .map(|dirs| dirs.contains(&direction))
            .unwrap_or(false)
        {
            // Hit a loop
            return None;
        }

        visited
            .entry(position)
            .or_insert_with(HashSet::new)
            .insert(direction);

        let in_front = position + direction;
        if map.obstructions.contains(&in_front) {
            direction = direction.perp();
            continue;
        }

        position += direction;
    }

    Some(visited)
}

#[aoc(day6, part1)]
pub fn solve_part1(map: &Map) -> usize {
    get_visited(map).unwrap().len()
}

#[aoc(day6, part2)]
pub fn solve_part2(map: &Map) -> usize {
    let mut visited = get_visited(map).unwrap();
    visited.remove(&map.guard);

    visited
        .iter()
        .filter(|(&v, _)| {
            let mut new_obstructions = map.obstructions.clone();
            new_obstructions.insert(v);
            let new_map = Map {
                size: map.size,
                obstructions: new_obstructions,
                guard: map.guard,
            };

            get_visited(&new_map).is_none()
        })
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_day6() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let map = super::input_generator(input);
        dbg!(super::solve_part2(&map));
    }
}
