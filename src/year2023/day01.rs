use crate::Puzzle;

use anyhow::Result;

pub struct Day01<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Puzzle<'a> for Day01<'a> {
    type Sol1Type = Option<u64>;
    type Sol2Type = u64;

    fn parse(input: &'a str) -> Result<Self> {
        let lines = input.lines().filter(|line| !line.is_empty()).collect();
        Ok(Self { lines })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        todo!()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        todo!()
    }
}
