use crate::harness::PuzzleSolution;

pub struct Day01;

const DIGIT_WORDS: &[(&[u8], u8)] = &[
    (b"zero", 0),
    (b"one", 1),
    (b"two", 2),
    (b"three", 3),
    (b"four", 4),
    (b"five", 5),
    (b"six", 6),
    (b"seven", 7),
    (b"eight", 8),
    (b"nine", 9),
];

fn check_digit(line_bytes: &[u8], i: usize) -> Option<u8> {
    if line_bytes[i].is_ascii_digit() {
        return Some(line_bytes[i] - b'0');
    } else {
        for &(word, value) in DIGIT_WORDS {
            if line_bytes[i..].len() >= word.len() && &line_bytes[i..][..word.len()] == word {
                return Some(value);
            }
        }
    }

    None
}

impl PuzzleSolution for Day01 {
    type Output = u64;

    fn part1(&self, input: &str) -> Self::Output {
        input.lines().fold(0, |acc, line| {
            let first_digit = line.chars().find(|c| c.is_ascii_digit()).unwrap() as u8 - b'0';
            let last_digit = line.chars().rev().find(|c| c.is_ascii_digit()).unwrap() as u8 - b'0';

            let value = first_digit * 10 + last_digit;
            acc + value as u64
        })
    }

    fn part2(&self, input: &str) -> Self::Output {
        input.lines().fold(0, |acc, line| {
            let line_bytes: Vec<_> = line.bytes().collect();

            let mut first_digit = 0;
            for i in 0..line_bytes.len() {
                if let Some(digit) = check_digit(&line_bytes, i) {
                    first_digit = digit;
                    break;
                }
            }

            let mut second_digit = 0;
            for i in (0..line_bytes.len()).rev() {
                if let Some(digit) = check_digit(&line_bytes, i) {
                    second_digit = digit;
                    break;
                }
            }

            let value = first_digit * 10 + second_digit;
            acc + value as u64
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

    #[test]
    fn part1() {
        assert_eq!(Day01.part1(TEST_INPUT1), 142);
    }

    #[test]
    fn test_check_digit() {
        assert_eq!(check_digit(b"two1nine", 0), Some(2));
        assert_eq!(check_digit(b"two1nine", 1), None);
        assert_eq!(check_digit(b"two1nine", 2), None);
        assert_eq!(check_digit(b"two1nine", 3), Some(1));
        assert_eq!(check_digit(b"two1nine", 4), Some(9));
        assert_eq!(check_digit(b"two1nine", 5), None);
        assert_eq!(check_digit(b"two1nine", 6), None);
        assert_eq!(check_digit(b"two1nine", 7), None);
    }

    const TEST_INPUT2: &str = r#"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"#;

    #[test]
    fn part2() {
        assert_eq!(Day01.part2(TEST_INPUT2), 281);
    }
}
