pub trait PuzzleSolution {
    type Output;

    fn part1(&self, input: &str) -> Self::Output;

    fn part2(&self, input: &str) -> Self::Output;
}
