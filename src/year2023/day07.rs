use crate::Puzzle;
use anyhow::{bail, Context, Result};
use std::cmp::Ordering;
use std::iter::zip;

pub struct Day {
    hands: Vec<Hand>,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Ord, PartialOrd, PartialEq, Eq, Clone, Copy)]
enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

struct Hand {
    cards: Box<[Card; 5]>,
    // Using an array of counters is a good 30% faster overall than a HashMap
    card_counters: Box<[usize; 13]>,
    bid: u32,
}

impl Day {
    fn winnings(
        &self,
        card_strength: impl Fn(&Card) -> Card,
        card_stats: impl Fn(&Hand) -> (usize, usize),
    ) -> u32 {
        let mut hands: Vec<_> = self
            .hands
            .iter()
            .map(|hand| {
                let (num_counters, max_counter) = card_stats(hand);

                let type_ = match (num_counters, max_counter) {
                    (0, _) => HandType::FiveOfAKind, // For all J hands
                    (1, _) => HandType::FiveOfAKind,
                    (2, 4) => HandType::FourOfAKind,
                    (2, 3) => HandType::FullHouse,
                    (3, 3) => HandType::ThreeOfAKind,
                    (3, 2) => HandType::TwoPair,
                    (4, _) => HandType::OnePair,
                    (5, _) => HandType::HighCard,
                    _ => unreachable!(),
                };

                (hand, type_)
            })
            .collect();

        hands.sort_unstable_by(|(a, a_type), (b, b_type)| match a_type.cmp(&b_type) {
            Ordering::Equal => zip(a.cards.iter(), b.cards.iter())
                .find(|(a_card, b_card)| a_card != b_card)
                .map(|(a_card, b_card)| card_strength(a_card).cmp(&card_strength(b_card)))
                .unwrap_or(Ordering::Equal),
            res => res,
        });

        hands
            .iter()
            .enumerate()
            .map(|(rank, (hand, _))| hand.bid * (u32::try_from(rank).unwrap() + 1))
            .sum()
    }
}

const J_INDEX: usize = 12;

impl<'a> Puzzle<'a> for Day {
    type Sol1Type = u32;
    type Sol2Type = Self::Sol1Type;

    fn parse(input: &'a str) -> Result<Self> {
        let hands = input
            .lines()
            .map(|hand| {
                let (cards_str, bid) = hand
                    .split_once(' ')
                    .with_context(|| format!("Failed to split hand: `{hand}`"))?;
                let bid = bid.parse()?;
                let mut card_counters = Box::new([0; 13]);
                let mut cards = Box::new([Card::A; 5]);
                for (card, c) in zip(cards.iter_mut(), cards_str.chars()) {
                    let i;
                    (i, *card) = match c {
                        'A' => (0, Card::A),
                        'K' => (1, Card::K),
                        'Q' => (2, Card::Q),
                        'T' => (3, Card::T),
                        '9' => (4, Card::Nine),
                        '8' => (5, Card::Eight),
                        '7' => (6, Card::Seven),
                        '6' => (7, Card::Six),
                        '5' => (8, Card::Five),
                        '4' => (9, Card::Four),
                        '3' => (10, Card::Three),
                        '2' => (11, Card::Two),
                        'J' => (J_INDEX, Card::J),
                        _ => bail!("Unrecognized card `{c}`"),
                    };

                    card_counters[i] += 1;
                }

                Ok(Hand {
                    cards,
                    card_counters,
                    bid,
                })
            })
            .collect::<Result<_>>()?;

        Ok(Day { hands })
    }

    fn solve_problem_1(&self) -> Self::Sol1Type {
        let card_strength = |card: &Card| *card;

        let card_stats = |hand: &Hand| {
            let num_counters = hand
                .card_counters
                .iter()
                .filter(|&counter| *counter != 0)
                .count();
            let max_counter = hand.card_counters.iter().copied().max().unwrap_or_default();

            (num_counters, max_counter)
        };

        self.winnings(card_strength, card_stats)
    }

    fn solve_problem_2(&self) -> Self::Sol2Type {
        let card_strength = |card: &Card| match card {
            Card::J => Card::Joker,
            _ => *card,
        };

        let card_stats = |hand: &Hand| {
            let j_counter = hand.card_counters[J_INDEX];
            let card_counters_no_j = &hand.card_counters[..J_INDEX];

            let num_counters = card_counters_no_j
                .iter()
                .filter(|&counter| *counter != 0)
                .count();
            let max_counter =
                j_counter + card_counters_no_j.iter().copied().max().unwrap_or_default();

            (num_counters, max_counter)
        };

        self.winnings(card_strength, card_stats)
    }
}

#[cfg(test)]
mod tests {
    use crate::Puzzle;
    use indoc::indoc;

    #[test]
    fn test_example() {
        let input = std::fs::read_to_string("input/2023/07_example_1.txt").unwrap();
        let day = super::Day::parse(&input).unwrap();
        assert_eq!(day.solve_problem_1(), 6440);
        assert_eq!(day.solve_problem_2(), 5905);
    }

    #[test]
    fn test_j() {
        // - J is weakest
        // - an all J hand works
        // - a hand with two Js and three unique other cards works
        const INPUT: &str = indoc! {"
            JJJJJ 100
            22222 300
            KJJ62 1
        "};

        let day = super::Day::parse(INPUT).unwrap();
        assert_eq!(day.solve_problem_2(), 1101);
    }
}
