use crate::Puzzle;
use anyhow::Result;
use std::str::Chars;

pub struct Day01<'a> {
    lines: Vec<&'a str>,
}

/// An iterator over proceedingly shorter substrings of a string slice.
///
/// ```
/// let x = "abc";
///
/// let mut substrs = Substrings { iter: x.chars() };
///
/// assert_eq!(Some("abc"), substrs.next());
/// assert_eq!(Some("bc"), substrs.next());
/// assert_eq!(Some("c"), substrs.next());
/// assert_eq!(None, substrs.next());
/// ```
struct Substrings<'a> {
    iter: Chars<'a>,
}

impl<'a> Iterator for Substrings<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        let substr = self.iter.as_str();
        self.iter.next()?;
        Some(substr)
    }
}

/// An iterator over proceedingly shorter substrings of a string slice, starting form the end.
///
/// ```
/// let x = "abc";
///
/// let mut substrs = RSubstrings { iter: x.chars() };
///
/// assert_eq!(Some("abc"), substrs.next());
/// assert_eq!(Some("ab"), substrs.next());
/// assert_eq!(Some("a"), substrs.next());
/// assert_eq!(None, substrs.next());
/// ```
struct RSubstrings<'a> {
    iter: Chars<'a>,
}

impl<'a> Iterator for RSubstrings<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        let substr = self.iter.as_str();
        self.iter.next_back()?;
        Some(substr)
    }
}

impl<'a> Puzzle<'a> for Day01<'a> {
    type Sol1Type = u32;
    type Sol2Type = Self::Sol1Type;

    fn parse(input: &'a str) -> Result<Self> {
        let lines = input.lines().collect();
        Ok(Self { lines })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        self.lines
            .iter()
            .map(|line| {
                let first_digit = line
                    .bytes()
                    .find(|c| matches!(c, b'0'..=b'9'))
                    .map(|c| c - b'0')
                    .unwrap_or_default();
                let last_digit = line
                    .bytes()
                    .rev()
                    .find(|c| matches!(c, b'0'..=b'9'))
                    .map(|c| c - b'0')
                    .unwrap_or_default();

                Self::Sol1Type::from(first_digit * 10 + last_digit)
            })
            .sum()
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        const DIGITS: [&str; 9] = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];

        macro_rules! find_digit {
            ($s:ident, $get_byte:ident, $match_at:ident) => {{
                if let Some(x @ b'0'..=b'9') = $s.as_bytes().$get_byte() {
                    return Some(x - b'0');
                }

                DIGITS
                    .iter()
                    .enumerate()
                    .find(|(_, &digit)| $s.$match_at(digit))
                    .map(|(i, _)| i as u8 + 1)
            }};
        }

        self.lines
            .iter()
            .map(|line| {
                let first_digit = Substrings { iter: line.chars() }
                    .find_map(|substr| find_digit!(substr, first, starts_with))
                    .unwrap_or_default();

                let last_digit = RSubstrings { iter: line.chars() }
                    .find_map(|substr| find_digit!(substr, last, ends_with))
                    .unwrap_or_default();

                Self::Sol2Type::from(first_digit * 10 + last_digit)
            })
            .sum()
    }
}
