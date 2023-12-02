use crate::harness::PuzzleSolution;

pub struct Day02;

#[derive(Debug)]
struct CubeSet {
    red: u64,
    green: u64,
    blue: u64,
}

impl CubeSet {
    fn parse(input: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for part in input.split(", ") {
            let mut parts = part.split(' ');
            let count = parts.next().unwrap().parse().unwrap();
            let color = parts.next().unwrap();

            match color {
                "red" => red = count,
                "green" => green = count,
                "blue" => blue = count,
                _ => panic!("invalid cube color `{color}'"),
            }
        }

        Self { red, green, blue }
    }

    fn is_possible(&self, max_red: u64, max_green: u64, max_blue: u64) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }
}

#[derive(Debug)]
struct Game {
    id: u64,
    cube_sets: Vec<CubeSet>,
}

impl Game {
    fn parse(input: &str) -> Self {
        let mut parts = input.split(": ");
        let game_id = {
            let mut parts = parts.next().unwrap().split(' ');
            let game_word = parts.next().unwrap();
            assert_eq!(game_word, "Game");
            parts.next().unwrap().parse().unwrap()
        };

        let cube_sets = parts
            .next()
            .unwrap()
            .split("; ")
            .map(CubeSet::parse)
            .collect();

        Self {
            id: game_id,
            cube_sets,
        }
    }

    fn is_possible(&self, max_red: u64, max_green: u64, max_blue: u64) -> bool {
        self.cube_sets
            .iter()
            .all(|c| c.is_possible(max_red, max_green, max_blue))
    }

    fn power(&self) -> u64 {
        let mut min_red = 0;
        let mut min_green = 0;
        let mut min_blue = 0;
        for set in &self.cube_sets {
            min_red = min_red.max(set.red);
            min_green = min_green.max(set.green);
            min_blue = min_blue.max(set.blue);
        }
        min_red * min_green * min_blue
    }
}

impl PuzzleSolution for Day02 {
    type Output = u64;

    fn part1(&self, input: &str) -> Self::Output {
        input.lines().fold(0, |acc, line| {
            let game = Game::parse(line);
            if game.is_possible(12, 13, 14) {
                acc + game.id
            } else {
                acc
            }
        })
    }

    fn part2(&self, input: &str) -> Self::Output {
        input.lines().fold(0, |acc, line| {
            let game = Game::parse(line);
            acc + game.power()
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;

    #[test]
    fn part1() {
        assert_eq!(Day02.part1(TEST_INPUT), 8);
    }

    #[test]
    fn part2() {
        assert_eq!(Day02.part2(TEST_INPUT), 2286);
    }
}
