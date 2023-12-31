use crate::Puzzle;
use anyhow::{Context, Result};
use std::str::FromStr;

pub struct Day {
    cards: Vec<Card>,
}

struct Card {
    winners: Vec<u32>,
    draws: Vec<u32>,
}

impl Card {
    fn count_matches(&self) -> usize {
        self.draws
            .iter()
            .filter(|draw| self.winners.contains(draw))
            .count()
    }
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (_, numbers) = s
            .split_once(": ")
            .with_context(|| format!("Failed to split card: `{s}`"))?;
        let (winners, draws) = numbers
            .split_once(" | ")
            .with_context(|| format!("Failed to split numbers: `{numbers}`"))?;

        let winners = winners
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()
            .with_context(|| format!("Failed to parse winners: `{winners}`"))?;
        let draws = draws
            .split(" ")
            .filter(|n| !n.is_empty())
            .map(str::parse)
            .collect::<Result<_, _>>()
            .with_context(|| format!("Failed to parse draws: `{draws}`"))?;

        Ok(Card { winners, draws })
    }
}

impl Puzzle<'_> for Day {
    type Sol1Type = u64;
    type Sol2Type = Self::Sol1Type;

    fn parse(input: &str) -> Result<Self> {
        let cards = input.lines().map(str::parse).collect::<Result<_>>()?;
        Ok(Day { cards })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        self.cards
            .iter()
            .map(|card| {
                let matches = card.count_matches();

                match matches {
                    0 => 0,
                    _ => 1 << (matches - 1),
                }
            })
            .sum()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        let mut counts = vec![1; self.cards.len()];

        for (i, cur) in self.cards.iter().enumerate() {
            let matches = cur.count_matches();
            let cur_copies = counts[i];
            for count in counts[(i + 1)..(i + 1 + matches)].iter_mut() {
                *count = *count + cur_copies;
            }
        }

        counts.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Puzzle;

    #[test]
    fn test_example() {
        let input = std::fs::read_to_string("input/2023/04_example_1.txt").unwrap();
        let day = super::Day::parse(&input).unwrap();
        assert_eq!(day.solve_problem_1(), 13);
        assert_eq!(day.solve_problem_2(), 30);
    }
}
