#![allow(
    clippy::derive_ord_xor_partial_ord,
    clippy::non_canonical_partial_ord_impl
)]

use std::cmp::{Ordering, Reverse};
use std::collections::HashMap;
use std::ops::Deref;

const INPUT: &str = include_str!("input");
const TEST_INPUT: &str = include_str!("test_input");

#[derive(Copy, Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
enum Card {
    A = 12,
    K = 11,
    Q = 10,
    J = 9,
    T = 8,
    Nine = 7,
    Eight = 6,
    Seven = 5,
    Six = 4,
    Five = 3,
    Four = 2,
    Three = 1,
    Two = 0,
}
impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            'A' => Self::A,
            'K' => Self::K,
            'Q' => Self::Q,
            'J' => Self::J,
            'T' => Self::T,
            '9' => Self::Nine,
            '8' => Self::Eight,
            '7' => Self::Seven,
            '6' => Self::Six,
            '5' => Self::Five,
            '4' => Self::Four,
            '3' => Self::Three,
            '2' => Self::Two,
            _ => panic!("Invalid card: {}", value),
        }
    }
}
impl Card {
    pub fn compare_with_joker(&self, other: &Self) -> Ordering {
        let self_val = match self {
            Card::J => 0,
            x => *x as u8 + 1,
        };
        let other_val = match other {
            Card::J => 0,
            x => *x as u8 + 1,
        };

        self_val.cmp(&other_val)
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Debug, Copy, Clone)]
enum HandType {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Hand([Card; 5]);
impl Hand {
    pub fn parse(input: &str) -> Self {
        let mut cards = [Card::Two; 5];
        for (index, char) in input.chars().enumerate() {
            cards[index] = char.into();
        }
        Self(cards)
    }

    pub fn hand_counts(&self) -> HashMap<Card, u8> {
        let mut counts = HashMap::new();
        for card in self.0.iter() {
            *counts.entry(*card).or_insert(0) += 1;
        }
        counts
    }

    pub fn hand_type(&self, jokers: bool) -> HandType {
        let mut counts = self.hand_counts();
        let joker_count = if jokers {
            let out = counts.get(&Card::J).copied().unwrap_or(0);
            counts.remove(&Card::J);
            out
        } else {
            0
        };
        if counts.values().any(|v| *v >= 5 - joker_count) || joker_count >= 5 {
            HandType::FiveOfAKind
        } else if counts.iter().any(|(_, v)| *v >= 4 - joker_count) {
            HandType::FourOfAKind
        } else if let Some((card, count)) = counts.iter().find(|(_, v)| **v >= 3 - joker_count) {
            let used_jokers = 3 - count;
            if counts
                .iter()
                .filter(|(c, _)| *c != card)
                .any(|(_, v)| *v >= 2 - joker_count + used_jokers)
            {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        } else {
            let mut used_jokers = 0;
            let mut twos = counts
                .iter()
                .filter(|(_, v)| {
                    let jokers_to_use = 2u8.saturating_sub(**v).min(joker_count - used_jokers);
                    if **v == 2 - jokers_to_use {
                        used_jokers += jokers_to_use;
                        true
                    } else {
                        false
                    }
                })
                .map(|(c, _)| c)
                .collect::<Vec<_>>();
            if twos.len() == 2 {
                twos.sort_by_key(|c| Reverse(*c));
                HandType::TwoPair
            } else if twos.len() == 1 {
                HandType::OnePair
            } else {
                HandType::HighCard
            }
        }
    }

    pub fn cmp_with_jokers(&self, other: &Self) -> Ordering {
        if self == other {
            return Ordering::Equal;
        }
        let type_compare = self.hand_type(true).cmp(&other.hand_type(true));
        if type_compare != Ordering::Equal {
            type_compare
        } else {
            let mut self_iter = self.iter();
            let mut other_iter = other.iter();
            loop {
                let next = self_iter.next();
                let result = next.unwrap().compare_with_joker(other_iter.next().unwrap());
                if result != Ordering::Equal {
                    return result;
                }
            }
        }
    }
}
impl Deref for Hand {
    type Target = [Card; 5];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }
        let type_compare = self.hand_type(false).cmp(&other.hand_type(false));
        if type_compare != Ordering::Equal {
            Some(type_compare)
        } else {
            let mut self_iter = self.iter();
            let mut other_iter = other.iter();
            loop {
                let result = self_iter.next().unwrap().cmp(other_iter.next().unwrap());
                if result != Ordering::Equal {
                    return Some(result);
                }
            }
        }
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[derive(Debug)]
pub struct HandAndBid {
    hand: Hand,
    bid: u64,
}
impl HandAndBid {
    pub fn parse(input: &str) -> Self {
        let mut split = input.split(' ');
        Self {
            hand: Hand::parse(split.next().unwrap()),
            bid: split.next().unwrap().parse().unwrap(),
        }
    }
}

pub fn day7() {
    let mut hands_and_bids = INPUT.lines().map(HandAndBid::parse).collect::<Vec<_>>();
    hands_and_bids.sort_by_key(|x| x.hand);
    let sum = hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, HandAndBid { bid, .. })| (i + 1) as u64 * *bid)
        .sum::<u64>();
    println!("Day 7 part 1: {}", sum);

    let mut hands_and_bids = INPUT.lines().map(HandAndBid::parse).collect::<Vec<_>>();
    hands_and_bids.sort_by(|x, y| x.hand.cmp_with_jokers(&y.hand));
    let sum = hands_and_bids
        .iter()
        .enumerate()
        .map(|(i, HandAndBid { bid, .. })| (i + 1) as u64 * *bid)
        .sum::<u64>();
    println!("Day 7 part 2: {}", sum);
}
