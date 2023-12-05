use std::collections::{HashMap, HashSet};

use crate::harness::PuzzleSolution;

pub struct Day04;

impl PuzzleSolution for Day04 {
    type Output = u64;

    fn part1(&self, input: &str) -> Self::Output {
        input.lines().fold(0, |acc, line| {
            let mut parts = line.split(": ");
            let numbers = parts.nth(1).unwrap();

            let mut parts = numbers.split(" | ");
            let winning_numbers: HashSet<&str> = parts.next().unwrap().split_whitespace().collect();
            let winning_count = parts
                .next()
                .unwrap()
                .split_whitespace()
                .filter(|num| winning_numbers.contains(num))
                .count();

            if winning_count > 0 {
                acc + 2u64.pow(winning_count as u32 - 1)
            } else {
                acc
            }
        })
    }

    fn part2(&self, input: &str) -> Self::Output {
        let win_record: Vec<_> = input
            .lines()
            .map(|line| {
                let mut parts = line.split(": ");
                let numbers = parts.nth(1).unwrap();

                let mut parts = numbers.split(" | ");
                let winning_numbers: HashSet<&str> =
                    parts.next().unwrap().split_whitespace().collect();

                parts
                    .next()
                    .unwrap()
                    .split_whitespace()
                    .filter(|num| winning_numbers.contains(num))
                    .count()
            })
            .collect();

        let mut card_copies = HashMap::new();
        for (i, &times_won) in win_record.iter().enumerate() {
            let copies_of_this = card_copies.entry(1 + i).or_insert(0);
            *copies_of_this += 1;
            for _ in 0..*copies_of_this {
                for j in 0..times_won {
                    *card_copies.entry(1 + i + j + 1).or_insert(0) += 1;
                }
            }
        }

        card_copies.values().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"#;

    #[test]
    fn part1() {
        assert_eq!(Day04.part1(TEST_INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(Day04.part2(TEST_INPUT), 30);
    }
}
