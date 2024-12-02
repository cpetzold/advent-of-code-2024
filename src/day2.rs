use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|report| is_report_safe(report))
        .count()
}

#[aoc(day2, part2)]
pub fn solve_part2(reports: &Vec<Vec<i32>>) -> usize {
    reports
        .iter()
        .filter(|&report| {
            if is_report_safe(report) {
                return true;
            }

            for i in 0..report.len() {
                let mut report_without_i = report.clone();
                report_without_i.remove(i);
                if is_report_safe(&report_without_i) {
                    return true;
                }
            }
            false
        })
        .count()
}

fn is_report_safe(report: &Vec<i32>) -> bool {
    let pair_diffs = report.windows(2).map(|w| w[0] - w[1]).collect::<Vec<i32>>();
    let signum = pair_diffs[0].signum();
    pair_diffs.iter().all(|diff| {
        let dist = diff.abs();
        diff.signum() == signum && dist >= 1 && dist <= 3
    })
}
