use crate::Puzzle;
use anyhow::{bail, Context, Result};
use std::str::FromStr;

pub struct Day02 {
    games: Vec<Game>,
}

struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn max_cube_numbers(&self) -> (u32, u32, u32) {
        use std::cmp::max;
        self.sets.iter().fold(
            (0, 0, 0),
            |(acc_r, acc_g, acc_b), &GameSet { red, green, blue }| {
                (max(acc_r, red), max(acc_g, green), max(acc_b, blue))
            },
        )
    }
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

        let sets = sets.split("; ").map(str::parse).collect::<Result<_>>()?;

        Ok(Game { id, sets })
    }
}

impl FromStr for GameSet {
    type Err = anyhow::Error;

    fn from_str(set: &str) -> Result<Self> {
        let (red, green, blue) =
            set.split(", ")
                .try_fold((0, 0, 0), |(red, green, blue), color| {
                    let (count, color) = color
                        .split_once(" ")
                        .with_context(|| format!("Failed to split color: `{color}`"))?;
                    let count = count
                        .parse()
                        .with_context(|| format!("Failed to parse color count: `{count}`"))?;

                    Ok(match color {
                        "red" => (count, green, blue),
                        "green" => (red, count, blue),
                        "blue" => (red, green, count),
                        _ => bail!("Unrecognized color {}", color),
                    })
                })?;

        Ok(GameSet { red, green, blue })
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
            .filter(|game| {
                let (max_red, max_green, max_blue) = game.max_cube_numbers();

                max_red <= RED_LIMIT && max_green <= GREEN_LIMIT && max_blue <= BLUE_LIMIT
            })
            .map(|game| game.id)
            .sum()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        self.games
            .iter()
            .map(|game| {
                let (max_red, max_green, max_blue) = game.max_cube_numbers();
                max_red * max_green * max_blue
            })
            .sum()
    }
}
