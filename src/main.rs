use std::{fmt::Display, time::Instant};

use harness::PuzzleSolution;

mod day01;
mod day02;
mod day03;
mod day04;
mod harness;

fn print_solution(day_num: usize, solution: impl PuzzleSolution<Output = impl Display>) {
    let input = std::fs::read_to_string(format!("inputs/day{:02}.txt", day_num)).unwrap();

    let (part1, part1_time) = {
        let start = Instant::now();
        let part1 = solution.part1(&input);
        (part1, start.elapsed())
    };
    let (part2, part2_time) = {
        let start = Instant::now();
        let part2 = solution.part2(&input);
        (part2, start.elapsed())
    };

    println!(
        "Day {}\n\tPart 1: {}\t({} ms)\n\tPart 2: {}\t({} ms)",
        day_num,
        part1,
        part1_time.as_micros() as f64 / 1000.0,
        part2,
        part2_time.as_micros() as f64 / 1000.0
    );
}

fn main() {
    let do_day = |day| match day {
        1 => print_solution(day, day01::Day01),
        2 => print_solution(day, day02::Day02),
        3 => print_solution(day, day03::Day03),
        4 => print_solution(day, day04::Day04),
        _ => panic!("invalid day `{day}`"),
    };

    if let Some(day) = std::env::args().nth(1) {
        let day = day.parse().unwrap();
        do_day(day);
    } else {
        for day in 1..5 {
            do_day(day);
        }
    }
}
