use crate::Puzzle;
use anyhow::{bail, Context, Result};
use std::str::FromStr;

pub struct Day02 {
    games: Vec<Game>,
}

struct Game {
    id: u32,
    sets: GameSets,
}

struct GameSets {
    reds: Vec<u32>,
    greens: Vec<u32>,
    blues: Vec<u32>,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (id, sets) = s
            .split_once(": ")
            .with_context(|| format!("Failed to split game: `{s}`"))?;
        let (_, id) = id
            .split_once(" ")
            .with_context(|| format!("Failed to split game ID: `{id}`"))?;
        let id = id
            .parse()
            .with_context(|| format!("Failed to parse game ID: `{id}`"))?;

        let sets = sets.parse()?;

        Ok(Game { id, sets })
    }
}

impl FromStr for GameSets {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut reds = Vec::new();
        let mut greens = Vec::new();
        let mut blues = Vec::new();

        for set in s.split("; ") {
            for color in set.split(", ") {
                let (count, color) = color
                    .split_once(" ")
                    .with_context(|| format!("Failed to split color: `{color}`"))?;
                let count = count
                    .parse()
                    .with_context(|| format!("Failed to parse color count: `{count}`"))?;

                match color {
                    "red" => reds.push(count),
                    "green" => greens.push(count),
                    "blue" => blues.push(count),
                    _ => bail!("Unrecognized color {}", color),
                }
            }
        }

        Ok(GameSets {
            reds,
            greens,
            blues,
        })
    }
}

impl Puzzle<'_> for Day02 {
    type Sol1Type = u32;
    type Sol2Type = Self::Sol1Type;

    fn parse(input: &str) -> Result<Self> {
        let games = input.lines().map(str::parse).collect::<Result<_>>()?;
        Ok(Day02 { games })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        const RED_LIMIT: u32 = 12;
        const GREEN_LIMIT: u32 = 13;
        const BLUE_LIMIT: u32 = 14;

        self.games
            .iter()
            .filter_map(|Game { id, sets }| {
                let max_red = sets.reds.iter().max().copied().unwrap_or_default();
                let max_green = sets.greens.iter().max().copied().unwrap_or_default();
                let max_blue = sets.blues.iter().max().copied().unwrap_or_default();

                (max_red <= RED_LIMIT && max_green <= GREEN_LIMIT && max_blue <= BLUE_LIMIT)
                    .then(|| *id)
            })
            .sum()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        self.games
            .iter()
            .map(|Game { sets, .. }| {
                let min_red = sets.reds.iter().max().copied().unwrap_or_default();
                let min_green = sets.greens.iter().max().copied().unwrap_or_default();
                let min_blue = sets.blues.iter().max().copied().unwrap_or_default();

                min_red * min_green * min_blue
            })
            .sum()
    }
}
