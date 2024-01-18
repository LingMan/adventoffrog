use crate::Puzzle;
use anyhow::{Context, Result};
use std::iter;

pub struct Day {
    races: Vec<Race>,
    combined_race: Race,
}

struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn num_possible_winning_moves(&self) -> u32 {
        // The problem can be represented by the inequality (T-t)t > D,
        // where T is the time and D is the distance.
        // Solving for t, first it can be transformed to:
        // t² - Tt + D < 0
        //
        // Then, by applying the p-q-formula with p = -T and q = D, we get:
        // t = -(-T)/2 ± sqrt((-T/2)² - D)
        //
        // Written a bit nicer as:
        // t = 0.5 * (T ± sqrt(T² - 4D))

        let time = self.time as f64;
        let distance = self.distance as f64;

        let root = (time * time - 4.0 * distance).sqrt();
        let lower_bound = 0.5 * (time - root);
        let upper_bound = 0.5 * (time + root);

        // If there's no integer between lower_bound and upper_bound, no solution exists in the
        // natural numbers. Should both bounds happen to be equal and to represent an integer,
        // there's still no solution, since we need to beat the record. Merely matching it doesn't
        // suffice.
        // If the square root is NaN, there's no solution even in the real numbers. Needs to be
        // checked separately since the NaN will propagate to both bounds but NaN != NaN.
        if lower_bound.trunc() == upper_bound.trunc() || root.is_nan() {
            return 0;
        }

        // Convert the result from real numbers to integers.
        // Need to make sure a lower bound of e.g. 4.0 gets bumped to 5, since the inequality
        // is `> D`, not `>= D`.
        let lower_bound = lower_bound.floor() as u32 + 1;
        let upper_bound = upper_bound.ceil() as u32 - 1;

        upper_bound - lower_bound + 1
    }
}

impl<'a> Puzzle<'a> for Day {
    type Sol1Type = u32;
    type Sol2Type = Self::Sol1Type;

    fn parse(input: &str) -> Result<Self> {
        let (times, distances) = input
            .split_once('\n')
            .with_context(|| format!("Failed to split lines: `{input}`"))?;
        let (_, times) = times
            .split_once(':')
            .with_context(|| format!("Failed to split times: `{times}`"))?;
        let (_, distances) = distances
            .split_once(':')
            .with_context(|| format!("Failed to split times: `{distances}`"))?;

        let combined_time = times.split_ascii_whitespace().collect::<String>().parse()?;
        let combined_distance = distances
            .split_ascii_whitespace()
            .collect::<String>()
            .parse()?;

        let combined_race = Race {
            time: combined_time,
            distance: combined_distance,
        };

        let times = times
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;
        let distances = distances
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()?;

        let races = iter::zip(times, distances)
            .map(|(time, distance)| Race { time, distance })
            .collect();

        Ok(Day {
            races,
            combined_race,
        })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        self.races
            .iter()
            .map(Race::num_possible_winning_moves)
            .product()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        self.combined_race.num_possible_winning_moves()
    }
}

#[cfg(test)]
mod tests {
    use crate::Puzzle;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = std::fs::read_to_string("input/2023/06_example_1.txt").unwrap();
        let day = super::Day::parse(&input).unwrap();
        assert_eq!(day.solve_problem_1(), 288);
        assert_eq!(day.solve_problem_2(), 71503);
    }

    #[test]
    fn test_u32_overflow() {
        const INPUT: &str = indoc! {"
            Time:        69     92     97     87
            Distance:   445   2182   3320   2329
        "};

        let day = super::Day::parse(INPUT).unwrap();
        assert_eq!(day.solve_problem_2(), 55761118);
    }
}
