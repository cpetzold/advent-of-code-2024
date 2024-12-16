use std::{collections::VecDeque, fmt::Display};

use aoc_runner_derive::{aoc, aoc_generator};
use glam::IVec2;
use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};
use vecgrid::Vecgrid;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Box,
    Robot,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            _ => unreachable!(),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Empty => ".",
                Tile::Wall => "#",
                Tile::Box => "O",
                Tile::Robot => "@",
            }
        )
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<Direction> for IVec2 {
    fn from(direction: Direction) -> Self {
        match direction {
            Direction::Up => IVec2::NEG_Y,
            Direction::Right => IVec2::X,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
        }
    }
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            '^' => Direction::Up,
            '>' => Direction::Right,
            'v' => Direction::Down,
            '<' => Direction::Left,
            _ => unreachable!(),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Direction::Up => "^",
                Direction::Right => ">",
                Direction::Down => "v",
                Direction::Left => "<",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Game {
    map: Vecgrid<Tile>,
    robot_pos: IVec2,
    movements: VecDeque<Direction>,
    last_move: Option<Direction>,
}

impl Game {
    fn get(&self, pos: IVec2) -> Option<Tile> {
        self.map.get(pos.y as usize, pos.x as usize).copied()
    }

    fn set(&mut self, pos: IVec2, tile: Tile) {
        self.map.set(pos.y as usize, pos.x as usize, tile).unwrap();
    }

    fn move_obstacle(&mut self, pos: IVec2, direction: Direction) -> bool {
        let Some(tile) = self.get(pos) else {
            return false;
        };

        let new_pos: IVec2 = pos + IVec2::from(direction);

        match self.get(new_pos) {
            Some(Tile::Empty) => {
                self.set(pos, Tile::Empty);
                self.set(new_pos, tile);
                true
            }
            Some(Tile::Box) => {
                if self.move_obstacle(new_pos, direction) {
                    self.set(pos, Tile::Empty);
                    self.set(new_pos, tile);
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }

    fn move_robot(&mut self, direction: Direction) -> bool {
        if self.move_obstacle(self.robot_pos, direction) {
            self.robot_pos += IVec2::from(direction);
            true
        } else {
            false
        }
    }

    fn step(&mut self) -> bool {
        let Some(direction) = self.movements.pop_front() else {
            return false;
        };
        self.last_move = Some(direction);
        self.move_robot(direction);
        true
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn box_positions(&self) -> Vec<IVec2> {
        self.map
            .enumerate_column_major()
            .filter(|(_, &tile)| tile == Tile::Box)
            .map(|(pos, _)| IVec2::new(pos.0 as i32, pos.1 as i32))
            .collect()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.map.column_len() {
            for x in 0..self.map.row_len() {
                write!(f, "{}", self.map.get(y, x).unwrap_or(&Tile::Empty))?;
            }
            if y < self.map.column_len() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

fn parse_map(input: &str) -> IResult<&str, Vecgrid<Tile>> {
    let (input, rows) =
        separated_list1(newline, many1(one_of(".#O@").map(|c| Tile::from(c))))(input)?;
    Ok((input, Vecgrid::from_rows(rows).unwrap()))
}

fn parse_movements(input: &str) -> IResult<&str, VecDeque<Direction>> {
    separated_list1(newline, many1(one_of("^>v<").map(|c| Direction::from(c))))(input)
        .map(|(input, movements)| (input, VecDeque::from_iter(movements.into_iter().flatten())))
}

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> Game {
    separated_pair(parse_map, many1(newline), parse_movements)(input)
        .map(|(_, (map, movements))| Game {
            map: map.clone(),
            robot_pos: map
                .enumerate_column_major()
                .find(|(_, &tile)| tile == Tile::Robot)
                .map(|(pos, _)| IVec2::new(pos.0 as i32, pos.1 as i32))
                .unwrap(),
            movements,
            last_move: None,
        })
        .unwrap()
}

#[aoc(day15, part1)]
pub fn solve_part1(game: &Game) -> i32 {
    let mut game = game.clone();
    game.run();

    game.box_positions()
        .iter()
        .map(|pos| 100 * pos.x + pos.y)
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_day14_input_generator() {
        let input = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        let game = input_generator(input);
        assert_eq!(
            game.to_string(),
            "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########"
        )
    }

    #[test]
    fn test_day15_part1() {
        let input = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";
        let mut game = input_generator(input);
        game.run();
        println!("{}", game);
        assert_eq!(
            game.to_string(),
            "##########
#.O.O.OOO#
#........#
#OO......#
#OO@.....#
#O#.....O#
#O.....OO#
#O.....OO#
#OO....OO#
##########"
        )
    }
}
