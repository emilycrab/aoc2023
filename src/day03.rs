use std::collections::{HashMap, HashSet};

use crate::harness::PuzzleSolution;

pub struct Day03;

impl PuzzleSolution for Day03 {
    type Output = u64;

    fn part1(&self, input: &str) -> Self::Output {
        let mut nums: Vec<(i64, i64, i64, u64)> = vec![];
        let mut symbols: HashSet<(i64, i64)> = HashSet::new();

        let mut current_num = 0;
        let mut current_num_start = None;
        let mut j = 0;
        let mut i = 0;
        for c in input.bytes() {
            if c.is_ascii_digit() {
                if current_num_start.is_none() {
                    current_num_start = Some(i);
                }
                let digit = (c - b'0') as u64;
                current_num = current_num * 10 + digit;
            } else {
                if let Some(current_num_start) = current_num_start.take() {
                    let len = i - current_num_start;
                    nums.push((j as i64, current_num_start as i64, len as i64, current_num));
                    current_num = 0;
                }

                if c.is_ascii_punctuation() && c != b'.' {
                    symbols.insert((j as i64, i as i64));
                }
            }

            if c == b'\n' {
                j += 1;
                i = 0;
            } else {
                i += 1;
            }
        }

        let mut sum = 0;
        for (row, col, len, value) in nums {
            for dj in -1..2 {
                for di in -1..(len + 1) {
                    if symbols.contains(&(row + dj, col + di)) {
                        sum += value;
                    }
                }
            }
        }

        sum
    }

    fn part2(&self, input: &str) -> Self::Output {
        let mut nums: Vec<(i64, i64, i64, u64)> = vec![];
        let mut gears: HashSet<(i64, i64)> = HashSet::new();

        let mut current_num = 0;
        let mut current_num_start = None;
        let mut j = 0;
        let mut i = 0;
        for c in input.bytes() {
            if c.is_ascii_digit() {
                if current_num_start.is_none() {
                    current_num_start = Some(i);
                }
                let digit = (c - b'0') as u64;
                current_num = current_num * 10 + digit;
            } else {
                if let Some(current_num_start) = current_num_start.take() {
                    let len = i - current_num_start;
                    nums.push((j as i64, current_num_start as i64, len as i64, current_num));
                    current_num = 0;
                }

                if c == b'*' {
                    gears.insert((j as i64, i as i64));
                }
            }

            if c == b'\n' {
                j += 1;
                i = 0;
            } else {
                i += 1;
            }
        }

        let mut nums_for_gear: HashMap<(i64, i64), Vec<u64>> = HashMap::new();
        for (row, col, len, value) in nums {
            for dj in -1..2 {
                for di in -1..(len + 1) {
                    if gears.contains(&(row + dj, col + di)) {
                        nums_for_gear
                            .entry((row + dj, col + di))
                            .and_modify(|e| e.push(value))
                            .or_insert(vec![value]);
                    }
                }
            }
        }

        let mut sum = 0;

        for nums in nums_for_gear.values() {
            if nums.len() == 2 {
                sum += nums[0] * nums[1];
            }
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598.."#;

    const TEST_INPUT2: &str = r#"...............487..........884....*......199.227........723...
.....937..608.................*....306........$................
......#....................128../.................%136....140..
.249.....590..$...949..768.......380......%...996..............
...+.854.../.678..*....................646...*.................
......*...........552........................741..........+....
.......976.............466......497$.............607......783..
............%....857...*....542.........915.543..$.............
....715......542..#..20....@........241...*...*.........701....
219*...................................#.599...845.....*.......
..........151.520...554.............................665........
.........*...........*.................687..............588....
..........463.....776......804............*..............*.....
....392..................................371...184........906.5
....*..........*......908..........497.........................
.125........862.326......&........+..........497.....&811......
......................%.....288...............*...-.......617..
.........787.........400.....*.......420....884.425......*.....
........*.......%...........596....=.*....................863..
.595=.112.737.699................382.888.......................
...........*......+.@......652.*...............................
..87......749..535...395..#....339..564.....813....410.........
...+........................$........+..967*.........*....921.."#;

    #[test]
    fn part1() {
        assert_eq!(Day03.part1(TEST_INPUT), 4361);
        assert_eq!(Day03.part1(TEST_INPUT2), 43104);
    }

    #[test]
    fn part2() {
        assert_eq!(Day03.part2(TEST_INPUT), 467835);
    }
}
