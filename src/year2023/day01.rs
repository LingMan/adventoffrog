use crate::Puzzle;

use anyhow::Result;

pub struct Day01<'a> {
    lines: Vec<&'a str>,
}

impl<'a> Puzzle<'a> for Day01<'a> {
    type Sol1Type = u32;
    type Sol2Type = Self::Sol1Type;

    fn parse(input: &'a str) -> Result<Self> {
        let lines = input.lines().collect();
        Ok(Self { lines })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        self.lines.iter()
            .map(|line| {
                let mut digits = line.bytes().filter(|c| c.is_ascii_digit());

                let first_digit = digits.next().map(|digit| digit - b'0').unwrap_or_default();

                let last_digit = digits
                    .next_back()
                    .map(|digit| digit - b'0')
                    .unwrap_or(first_digit);

                Self::Sol1Type::from(first_digit * 10 + last_digit)
            })
            .sum()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        todo!()
    }
}
