use crate::util::SplitEmptyLines;
use crate::Puzzle;
use anyhow::{bail, Context, Result};
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Range;
use std::str::FromStr;

pub struct Day<'a> {
    seeds: Vec<u64>,
    mappings: HashMap<&'a str, (&'a str, Vec<Mapping>)>,
}

struct Mapping {
    src: Range<u64>,
    dst: u64,
}

impl Mapping {
    fn map(&self, value: Range<u64>, mapped: &mut Vec<Range<u64>>, unmapped: &mut Vec<Range<u64>>) {
        let contains_start = self.src.contains(&value.start);
        let contains_end = self.src.contains(&(value.end - 1));

        // Put the "no overlap" case first, since it's by far the most common one
        if !contains_start && !contains_end {
            // No overlap
            unmapped.push(value);
        } else if contains_start && contains_end {
            // Input range fully contained within self.src
            let new_start = value.start - self.src.start + self.dst;
            let new_end = value.end - self.src.start + self.dst;

            mapped.push(new_start..new_end);
        } else if !contains_start && contains_end {
            // Partial overlap. x..value.end contained within self.src, but value.start..x is outside
            let new_start = self.dst;
            let new_end = value.end - self.src.start + self.dst;

            let before_start = value.start;
            let before_end = self.src.start;

            mapped.push(new_start..new_end);
            unmapped.push(before_start..before_end);
        } else if contains_start && !contains_end {
            // Partial overlap. value.start..x contained within self.src, but x..value.end is outside
            let new_start = value.start - self.src.start + self.dst;
            let new_end = self.dst + (self.src.end - self.src.start);

            let after_start = self.src.end;
            let after_end = value.end;

            mapped.push(new_start..new_end);
            unmapped.push(after_start..after_end);
        } else {
            // self.src fully contained within the input range
            let new_start = self.dst;
            let new_end = self.dst + (self.src.end - self.src.start);

            let before_start = value.start;
            let before_end = self.src.start;

            let after_start = self.src.end;
            let after_end = value.end;

            mapped.push(new_start..new_end);
            unmapped.push(before_start..before_end);
            unmapped.push(after_start..after_end);
        }
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (dst_start, s) = s
            .split_once(' ')
            .with_context(|| format!("Failed to split mapping: `{s}`"))?;
        let (src_start, length) = s
            .split_once(' ')
            .with_context(|| format!("Failed to split mapping: `{s}`"))?;

        let dst_start = dst_start
            .parse()
            .with_context(|| format!("Failed to parse dst_start: `{dst_start}`"))?;
        let src_start = src_start
            .parse()
            .with_context(|| format!("Failed to parse src_start: `{src_start}`"))?;
        let length: u64 = length
            .parse()
            .with_context(|| format!("Failed to parse length: `{length}`"))?;

        Ok(Mapping {
            src: src_start..(src_start + length),
            dst: dst_start,
        })
    }
}

fn compute_closest_seed_location(
    mut number_ranges: Vec<Range<u64>>,
    mappings: &HashMap<&str, (&str, Vec<Mapping>)>,
) -> Result<u64> {
    let mut src_types = vec!["seed"];

    loop {
        let mut mapped_ranges = Vec::new();
        // These two semantically belong inside the loops below, but reallocating them in each
        // iteration more than doubles the overall runtime.
        let mut unprocessed_ranges = Vec::new();
        let mut unmapped_ranges = Vec::new();

        let src_type = *src_types.last().unwrap();
        let Some((dst_type, mappings)) = mappings.get(src_type) else {
            bail!("Failed to find map for `{src_type}`");
        };

        for num in number_ranges.into_iter() {
            unprocessed_ranges.push(num);

            for mapping in mappings.iter() {
                for range in unprocessed_ranges.drain(..) {
                    mapping.map(range, &mut mapped_ranges, &mut unmapped_ranges)
                }

                std::mem::swap(&mut unprocessed_ranges, &mut unmapped_ranges);

                if unprocessed_ranges.is_empty() {
                    break;
                }
            }

            mapped_ranges.append(&mut unprocessed_ranges);
        }

        number_ranges = mapped_ranges;

        if *dst_type == "location" {
            break;
        }

        if src_types.contains(dst_type) {
            bail!("Conversion loop detected. Type `{dst_type}` was encounted twice.");
        }
        src_types.push(dst_type);
    }

    Ok(number_ranges.iter().map(|r| r.start).min().unwrap())
}

impl<'a> Puzzle<'a> for Day<'a> {
    type Sol1Type = Result<u64>;
    type Sol2Type = Self::Sol1Type;

    fn parse(input: &'a str) -> Result<Self> {
        let mut seeds = None;
        let mut mappings = HashMap::new();

        for block in input.split_empty_lines() {
            let (title, values) = block
                .split_once(':')
                .with_context(|| format!("Failed to split block: `{block}`"))?;
            if title == "seeds" {
                seeds = Some(
                    values
                        .split_ascii_whitespace()
                        .map(str::parse)
                        .collect::<Result<_, _>>()?,
                )
            } else {
                let Some((title, "map")) = title.split_once(' ') else {
                    bail!("Unrecognized title `{title}`");
                };

                let (src_type, dst_type) = title
                    .split_once("-to-")
                    .with_context(|| format!("Unrecognized map title `{title}`"))?;

                let mapping = values
                    .trim()
                    .lines()
                    .map(str::parse)
                    .collect::<Result<_>>()?;

                if mappings.insert(src_type, (dst_type, mapping)).is_some() {
                    bail!("Duplicate map for source type `{src_type}`");
                }
            }
        }
        Ok(Self {
            seeds: seeds.context("Seeds not found")?,
            mappings,
        })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        // Turn each seed into a range with length 1, so we can reuse the solution for part 2
        let seed_ranges = self.seeds.iter().map(|&seed| seed..seed + 1).collect();
        compute_closest_seed_location(seed_ranges, &self.mappings)
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        let seed_ranges = self
            .seeds
            .iter()
            .tuples()
            .map(|(&start, &len)| start..(start + len))
            .collect();
        compute_closest_seed_location(seed_ranges, &self.mappings)
    }
}

#[cfg(test)]
mod tests {
    use crate::Puzzle;

    #[test]
    fn test_example() {
        let input = std::fs::read_to_string("input/2023/05_example_1.txt").unwrap();
        let day = super::Day::parse(&input).unwrap();
        assert_eq!(day.solve_problem_1().unwrap(), 35);
        assert_eq!(day.solve_problem_2().unwrap(), 46);
    }
}
