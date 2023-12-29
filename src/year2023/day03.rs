use crate::Puzzle;
use anyhow::Result;
use std::ops::Range;

pub struct Day {
    part_nums: Vec<PartNumber>,
    symbols: Vec<Symbol>,
}

struct PartNumber {
    row: usize,
    columns: Range<usize>,
    value: u64,
}

impl PartNumber {
    fn is_adjacent_to(&self, s: &Symbol) -> bool {
        let start = self.columns.start.saturating_sub(1);
        let end = self.columns.end + 1;
        (start..end).contains(&s.col)
    }
}

struct Symbol {
    row: usize,
    col: usize,
    value: char,
}

trait HasRows {
    fn get_row(&self) -> usize;
}

impl HasRows for PartNumber {
    fn get_row(&self) -> usize {
        self.row
    }
}

impl HasRows for Symbol {
    fn get_row(&self) -> usize {
        self.row
    }
}

struct AdjacencyCandidates<'a, T> {
    slice: &'a [T],
    low: usize,
    high: usize,
    current_row: usize,
}

impl<'a, T: HasRows> AdjacencyCandidates<'a, T> {
    fn from(slice: &'a [T]) -> Self {
        AdjacencyCandidates {
            slice,
            low: 0,
            high: slice.len(),
            current_row: usize::MAX,
        }
    }

    fn get_for<U: HasRows>(&mut self, target: &U) -> &[T] {
        if self.current_row != target.get_row() {
            self.low = self
                .slice
                .partition_point(|t| t.get_row() < target.get_row().saturating_sub(1));
            self.high = self
                .slice
                .partition_point(|t| t.get_row() <= target.get_row() + 1);
            self.current_row = target.get_row();
        }
        &self.slice[self.low..self.high]
    }
}

impl Puzzle<'_> for Day {
    type Sol1Type = u64;
    type Sol2Type = u64;

    fn parse(input: &str) -> Result<Self> {
        let mut part_nums = Vec::new();
        let mut symbols = Vec::new();

        for (row, line) in input.lines().enumerate() {
            let mut num: Option<PartNumber> = None;

            macro_rules! terminate_number {
                () => {
                    if let Some(num) = num.take() {
                        part_nums.push(num);
                    }
                };
            }

            for (i, c) in line.char_indices() {
                match c {
                    '.' => terminate_number!(),
                    '0'..='9' => {
                        num = Some(match num {
                            Some(num) => PartNumber {
                                columns: num.columns.start..(num.columns.end + 1),
                                value: num.value * 10 + u64::from(c.to_digit(10).unwrap()),
                                ..num
                            },
                            None => PartNumber {
                                row,
                                columns: i..(i + 1),
                                value: c.to_digit(10).unwrap().into(),
                            },
                        });
                    }
                    _ => {
                        terminate_number!();
                        symbols.push(Symbol {
                            row,
                            col: i,
                            value: c,
                        });
                    }
                }
            }

            terminate_number!();
        }

        Ok(Day { part_nums, symbols })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        let mut candidates = AdjacencyCandidates::from(&self.symbols[..]);

        self.part_nums
            .iter()
            .filter(|part_num| {
                candidates
                    .get_for(*part_num)
                    .iter()
                    .any(|s| part_num.is_adjacent_to(s))
            })
            .map(|p| p.value)
            .sum()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        let mut candidates = AdjacencyCandidates::from(&self.part_nums[..]);

        self.symbols
            .iter()
            .filter(|s| s.value == '*')
            .map(|symbol| {
                let (count, gear_ratio) = candidates
                    .get_for(symbol)
                    .iter()
                    .filter(|p| p.is_adjacent_to(symbol))
                    .fold((0, 1), |(count, acc), p| (count + 1, acc * p.value));

                match count {
                    2 => gear_ratio,
                    _ => 0,
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::Puzzle;
    use indoc::indoc;

    const INPUT: &str = indoc! {"
        123+=.*7894
        56.633.../.
        ..*......21
    "};

    #[test]
    fn test_parsing() {
        let parsed: Vec<_> = super::Day::parse(INPUT)
            .unwrap()
            .part_nums
            .iter()
            .map(|num| num.value)
            .collect();
        let expected = [123, 7894, 56, 633, 21];

        assert_eq!(parsed, expected);
    }
}
