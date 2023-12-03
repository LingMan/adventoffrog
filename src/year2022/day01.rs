use std::fmt::Debug;
use std::str::FromStr;

use crate::Puzzle;

use anyhow::{Context, Result};
use arrayvec::ArrayVec;

pub struct Day01 {
    elfs: Vec<Elf>,
}

#[derive(Debug)]
struct Elf {
    inventory: Vec<Food>,
}

#[derive(Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
struct Food {
    calories: u64,
}

impl FromStr for Food {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(Food {
            calories: s
                .parse()
                .with_context(|| format!("Failed to read calory count from: `{s}`"))?,
        })
    }
}

fn sum_of_calories_per_elf(elfs: &[Elf]) -> impl Iterator<Item = u64> + '_ {
    elfs.iter()
        .map(|elf| elf.inventory.iter().map(|food| food.calories).sum())
}

impl Puzzle<'_> for Day01 {
    type Sol1Type = Option<u64>;
    type Sol2Type = u64;

    fn parse(input: &str) -> Result<Self> {
        let mut elfs = Vec::new();
        let mut lines = input.lines().peekable();
        while lines.peek().is_some() {
            let inventory = lines
                .by_ref()
                .take_while(|line| !line.is_empty())
                .map(str::parse)
                .collect::<Result<_>>()?;
            elfs.push(Elf { inventory });
        }

        Ok(Self { elfs })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        sum_of_calories_per_elf(&self.elfs).max()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        const NUM_TOP_SPOTS: usize = 3;

        let mut top = ArrayVec::<_, { NUM_TOP_SPOTS + 1 }>::new();
        for calories in sum_of_calories_per_elf(&self.elfs) {
            let i = top.partition_point(|&cur| cur > calories);
            top.insert(i, calories);
            top.truncate(NUM_TOP_SPOTS);
        }
        top.iter().sum()
    }
}
