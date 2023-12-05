use std::ops::Range;

use rayon::{
    iter::{ParallelBridge, ParallelIterator},
    slice::ParallelSlice,
};

use crate::harness::PuzzleSolution;

pub struct Day05;

struct CategoryMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl CategoryMap {
    fn parse(input: &str) -> Self {
        let mappings = input
            .lines()
            .skip(1)
            .map(|line| {
                let mut parts = line.split_whitespace();
                // Destination range start
                let drs = parts.next().unwrap().parse().unwrap();
                // Source range start
                let srs = parts.next().unwrap().parse().unwrap();
                // Range length
                let len: u64 = parts.next().unwrap().parse().unwrap();
                (drs..(drs + len), srs..(srs + len))
            })
            .collect();
        Self { mappings }
    }

    fn get_destination_id(&self, source_id: u64) -> u64 {
        for (dst_range, src_range) in &self.mappings {
            if src_range.contains(&source_id) {
                return dst_range.start + (source_id - src_range.start);
            }
        }
        source_id
    }
}

struct Almanac {
    seeds: Vec<u64>,
    category_maps: Vec<CategoryMap>,
}

impl Almanac {
    fn parse(input: &str) -> Self {
        let mut blocks = input.split("\n\n");
        let seeds = blocks
            .next()
            .unwrap()
            .split(": ")
            .nth(1)
            .unwrap()
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let category_maps = blocks.map(CategoryMap::parse).collect();
        Self {
            seeds,
            category_maps,
        }
    }

    fn get_lowest_location2(&self) -> u64 {
        self.seeds
            .par_chunks_exact(2)
            .flat_map_iter(|chunk| {
                let range_start = chunk[0];
                let range_len = chunk[1];
                range_start..(range_start + range_len)
            })
            .map(|seed| self.get_location(seed))
            .min()
            .unwrap()
    }

    fn get_lowest_location1(&self) -> u64 {
        self.seeds
            .iter()
            .map(|&seed| self.get_location(seed))
            .min()
            .unwrap()
    }

    fn get_location(&self, seed: u64) -> u64 {
        let mut category_id = seed;
        for map in &self.category_maps {
            category_id = map.get_destination_id(category_id);
        }
        category_id
    }
}

impl PuzzleSolution for Day05 {
    type Output = u64;

    fn part1(&self, input: &str) -> Self::Output {
        Almanac::parse(input).get_lowest_location1()
    }

    fn part2(&self, input: &str) -> Self::Output {
        Almanac::parse(input).get_lowest_location2()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"#;

    #[test]
    fn part1() {
        assert_eq!(Day05.part1(TEST_INPUT), 35);
    }

    #[test]
    fn part2() {
        assert_eq!(Day05.part2(TEST_INPUT), 46);
    }
}
