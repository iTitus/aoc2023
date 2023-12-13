use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn find(hand: &Hand) -> Self {
        let counts = hand.cards.iter().counts();
        let jokers = counts.get(&Card::Joker);
        match counts.len() {
            1 => Self::FiveOfAKind,
            2 => {
                if jokers.is_some() {
                    Self::FiveOfAKind
                } else if *counts.values().max().unwrap() == 4 {
                    Self::FourOfAKind
                } else {
                    Self::FullHouse
                }
            }
            3 => {
                if *counts.values().max().unwrap() == 3 {
                    if jokers.is_some() {
                        Self::FourOfAKind
                    } else {
                        Self::ThreeOfAKind
                    }
                } else {
                    match jokers {
                        None => Self::TwoPair,
                        Some(1) => Self::FullHouse,
                        Some(2) => Self::FourOfAKind,
                        _ => unreachable!(),
                    }
                }
            }
            4 => {
                if jokers.is_some() {
                    Self::ThreeOfAKind
                } else {
                    Self::OnePair
                }
            }
            5 => {
                if jokers.is_some() {
                    Self::OnePair
                } else {
                    Self::HighCard
                }
            }
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Card {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl TryFrom<char> for Card {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::Jack,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => {
                return Err(());
            }
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct Hand {
    cards: [Card; 5],
}

impl FromStr for Hand {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 5 || !s.is_ascii() {
            return Err(());
        }

        let cards = s
            .chars()
            .map(TryFrom::try_from)
            .process_results(|it| it.collect_vec().try_into())
            .map_err(|_| ())?
            .map_err(|_| ())?;
        Ok(Hand { cards })
    }
}

impl Hand {
    fn enable_joker(&mut self) {
        self.cards.iter_mut().for_each(|c| {
            if matches!(c, Card::Jack) {
                *c = Card::Joker;
            }
        })
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Bid {
    hand: Hand,
    bid: u32,
}

impl Bid {
    fn enable_joker(&mut self) {
        self.hand.enable_joker()
    }
}

impl FromStr for Bid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_whitespace().collect_tuple().ok_or(())?;
        Ok(Bid {
            hand: Hand::from_str(hand).map_err(|_| ())?,
            bid: bid.parse().map_err(|_| ())?,
        })
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Vec<Bid> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[Bid]) -> u32 {
    let mut bids = input.to_vec();
    bids.sort_by_cached_key(|b| (HandType::find(&b.hand), b.hand));
    bids.iter()
        .enumerate()
        .map(|(n, bid)| (n + 1) as u32 * bid.bid)
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &[Bid]) -> u32 {
    let mut bids = input.to_vec();
    bids.iter_mut().for_each(|b| b.enable_joker());
    bids.sort_by_cached_key(|b| (HandType::find(&b.hand), b.hand));
    bids.iter()
        .enumerate()
        .map(|(n, bid)| (n + 1) as u32 * bid.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    const INPUT: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    /// https://www.reddit.com/r/adventofcode/comments/18cr4xr/2023_day_7_better_example_input_not_a_spoiler/
    const INPUT_2: &str = r#"2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_generator(INPUT)), 6440)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_generator(INPUT)), 5905)
    }

    #[test]
    fn test_part1_2() {
        assert_eq!(part1(&input_generator(INPUT_2)), 6592)
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(part2(&input_generator(INPUT_2)), 6839)
    }
}
