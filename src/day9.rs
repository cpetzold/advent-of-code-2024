use std::fmt::Display;

use aoc_runner_derive::{aoc, aoc_generator};
use itertools::{repeat_n, Itertools};

type Block = Option<usize>;

pub struct Disk {
    blocks: Vec<Block>,
}

impl Disk {
    pub fn compact(&self) -> Disk {
        let mut blocks = self.blocks.clone();

        let mut i = 0;

        for (j, _) in blocks
            .clone()
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, b)| b.is_some())
        {
            while blocks[i].is_some() && i < blocks.len() {
                i += 1;
            }

            if i >= j {
                break;
            }

            blocks.swap(i, j);
        }

        Disk { blocks }
    }

    pub fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .map(|(i, b)| i * b.unwrap_or(0))
            .sum()
    }
}

impl Display for Disk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.blocks
                .iter()
                .map(|b| b.map_or(String::from("."), |b| b.to_string()))
                .join("")
        )
    }
}

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Disk {
    let blocks = input
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .enumerate()
        .flat_map(|(i, n)| {
            let block = if i % 2 == 0 { Some(i / 2) } else { None };
            repeat_n(block, n)
        })
        .collect();

    Disk { blocks }
}

#[aoc(day9, part1)]
pub fn solve_part1(disk: &Disk) -> usize {
    let compacted = disk.compact();
    compacted.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_input_generator() {
        let input = "12345";

        let disk = input_generator(&input);

        assert_eq!(format!("{}", disk), "0..111....22222");
    }

    #[test]
    fn test_day9_compact() {
        let input = "12345";

        let disk = input_generator(&input);
        let compacted = disk.compact();
        assert_eq!(format!("{}", disk), "0..111....22222");
        assert_eq!(format!("{}", compacted), "022111222......");
    }
}
