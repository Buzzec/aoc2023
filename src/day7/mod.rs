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

enum HandType {
    FiveOfAKind(Card),
    FourOfAKind { four: Card, kicker: Card },
    FullHouse { three: Card, two: Card },
    ThreeOfAKind { three: Card, kickers: [Card; 2] },
    TwoPair { high: Card, low: Card, kicker: Card },
    OnePair { pair: Card, kickers: [Card; 3] },
    HighCard([Card; 5]),
}
impl HandType {
    pub fn type_val(&self) -> usize {
        match self {
            HandType::FiveOfAKind(_) => 6,
            HandType::FourOfAKind { .. } => 5,
            HandType::FullHouse { .. } => 4,
            HandType::ThreeOfAKind { .. } => 3,
            HandType::TwoPair { .. } => 2,
            HandType::OnePair { .. } => 1,
            HandType::HighCard(_) => 0,
        }
    }

    pub fn compare_by_type(&self, other: &Self) -> Ordering {
        self.type_val().cmp(&other.type_val())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord)]
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

    pub fn hand_type(&self) -> HandType {
        let counts = self.hand_counts();
        if counts.values().any(|v| *v == 5) {
            HandType::FiveOfAKind(self.0[0])
        } else if let Some(card) = counts.iter().find(|(_, v)| **v == 4).map(|(c, _)| *c) {
            HandType::FourOfAKind {
                four: card,
                kicker: self.into_iter().find(|c| *c != card).unwrap(),
            }
        } else if let Some(card) = counts.iter().find(|(_, v)| **v == 3).map(|(c, _)| c) {
            let mut other = counts
                .iter()
                .filter(|(_, v)| **v != 3)
                .map(|(c, _)| *c)
                .collect::<Vec<_>>();
            if other.len() == 1 {
                HandType::FullHouse {
                    three: *card,
                    two: other[0],
                }
            } else {
                other.sort_by_key(|c| Reverse(*c));
                HandType::ThreeOfAKind {
                    three: *card,
                    kickers: other.try_into().unwrap(),
                }
            }
        } else {
            let mut twos = counts
                .iter()
                .filter(|(_, v)| **v == 2)
                .map(|(c, _)| c)
                .collect::<Vec<_>>();
            if twos.len() == 2 {
                twos.sort_by_key(|c| Reverse(*c));
                HandType::TwoPair {
                    high: *twos[0],
                    low: *twos[1],
                    kicker: *self.0.iter().find(|c| !twos.contains(c)).unwrap(),
                }
            } else if twos.len() == 1 {
                let mut other = counts
                    .iter()
                    .filter(|(_, v)| **v != 2)
                    .map(|(c, _)| *c)
                    .collect::<Vec<_>>();
                other.sort_by_key(|c| Reverse(*c));
                HandType::OnePair {
                    pair: *twos[0],
                    kickers: other.as_slice().try_into().unwrap(),
                }
            } else {
                let mut out = self.0.to_vec();
                out.sort_by_key(|c| Reverse(*c));
                HandType::HighCard(out.as_slice().try_into().unwrap())
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
        let type_compare = self.hand_type().compare_by_type(&other.hand_type());
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
}
