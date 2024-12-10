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

    pub fn compact_whole_files(&self) -> Disk {
        let mut blocks = self.blocks.clone();

        for (_, file_blocks) in &blocks
            .clone()
            .iter()
            .enumerate()
            .rev()
            .filter(|(_, b)| b.is_some())
            .chunk_by(|(_, b)| **b)
        {
            let mut i = 0;
            let mut file_blocks = file_blocks.collect_vec();
            file_blocks.reverse();
            let file_size = file_blocks.len();
            let file_start_index = file_blocks[0].0;

            while i < file_start_index && blocks[i..i + file_size].iter().any(|b| b.is_some()) {
                i += 1;
            }

            if i >= file_start_index {
                continue;
            }

            for (k, (j, _)) in file_blocks.iter().enumerate() {
                blocks.swap(i + k, *j);
            }
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

#[aoc(day9, part2)]
pub fn solve_part2(disk: &Disk) -> usize {
    let compacted = disk.compact_whole_files();
    compacted.checksum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day9_input_generator() {
        let input = "2333133121414131402";

        let disk = input_generator(&input);

        assert_eq!(
            format!("{}", disk),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }

    #[test]
    fn test_day9_compact() {
        let input = "2333133121414131402";

        let disk = input_generator(&input);
        let compacted = disk.compact();
        assert_eq!(
            format!("{}", compacted),
            "0099811188827773336446555566.............."
        );
        assert_eq!(compacted.checksum(), 1928);
    }

    #[test]
    fn test_day9_compact_whole_files() {
        let input = "2333133121414131402";

        let disk = input_generator(&input);
        let compacted = disk.compact_whole_files();
        assert_eq!(
            format!("{}", compacted),
            "00992111777.44.333....5555.6666.....8888.."
        );
        assert_eq!(compacted.checksum(), 2858);
    }
}
