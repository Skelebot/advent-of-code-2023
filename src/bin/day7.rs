use advent_of_code_2023::*;

fn part1(hands: &[Hand]) -> u32 {
    let mut hands = Vec::from(hands);
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum()
}

fn part2(hands: &[JokerHand]) -> u32 {
    let mut hands = Vec::from(hands);
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(i, h)| (i as u32 + 1) * h.bid)
        .sum()
}

fn main() {
    println!("Solution 1: {}", part1(&read_lines("in/day7")));
    println!("Solution 2: {}", part2(&read_lines("in/day7")));
}

#[test]
fn test() {
    assert_eq!(6440, part1(&read_lines("in/day7test")));
    assert_eq!(5905, part2(&read_lines("in/day7test")));
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    Five = 7,
    Four = 6,
    FullHouse = 5,
    Three = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    pub fn determine(hand: &[u8]) -> HandType {
        let mut threes = 0;
        let mut twos = 0;
        for c in hand.iter() {
            let same_count = hand.iter().filter(|d| *d == c).count();
            match same_count {
                5 => return HandType::Five,
                4 => return HandType::Four,
                3 => threes += 1,
                2 => twos += 1,
                _ => (),
            }
        }
        threes /= 3;
        twos /= 2;
        match (threes, twos) {
            (1, 1) => HandType::FullHouse,
            (1, 0) => HandType::Three,
            (0, 2) => HandType::TwoPair,
            (0, 1) => HandType::OnePair,
            (0, 0) => HandType::HighCard,
            _ => panic!("failed to determine hand type: {:?}", (threes, twos)),
        }
    }
    pub fn determine_joker(hand: &[u8]) -> HandType {
        let mut best_htype = Self::determine(hand);
        if hand.contains(&1) {
            let mut test_hand = Vec::from(hand);
            let j_idxs: Vec<usize> = test_hand
                .iter()
                .enumerate()
                .filter(|(_, c)| **c == 1)
                .map(|(i, _)| i)
                .collect();
            for val in (2..11).chain(12..15) {
                for i in j_idxs.iter() {
                    test_hand[*i] = val
                }
                let new_htype = Self::determine(&test_hand);
                if new_htype > best_htype {
                    best_htype = new_htype
                }
            }
        }
        best_htype
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<u8>,
    bid: u32,
    htype: HandType,
}

use std::cmp::Ordering::{self, *};
impl std::cmp::PartialOrd for Hand {
    fn partial_cmp(&self, other: &Hand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for Hand {
    fn cmp(&self, other: &Hand) -> Ordering {
        let res = self.htype.cmp(&other.htype);
        match res {
            Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

impl std::cmp::PartialEq for Hand {
    fn eq(&self, other: &Hand) -> bool {
        self.cards == other.cards
    }
}
impl std::cmp::Eq for Hand {}

#[test]
fn test_cmp() {
    let card1: Hand = "33332 1".parse().unwrap();
    let card2: Hand = "2AAAA 1".parse().unwrap();
    assert!(card1 > card2);
    let card1: Hand = "77888 1".parse().unwrap();
    let card2: Hand = "77788 1".parse().unwrap();
    assert!(card2 < card1);
}

impl std::str::FromStr for Hand {
    type Err = ();
    fn from_str(s: &str) -> Result<Hand, Self::Err> {
        let mut st = s.trim().split(' ');
        let cards_c = st.next().unwrap();
        let bid = st.next().unwrap().parse().unwrap();
        let mut cards = Vec::new();

        for c in cards_c.trim().chars() {
            let val = match c {
                'A' => 14,
                'K' => 13,
                'Q' => 12,
                'J' => 11,
                'T' => 10,
                x if x.is_ascii_digit() => x as u8 - b'0',
                _ => panic!("invalid card symbol"),
            };
            cards.push(val);
        }
        let htype = HandType::determine(&cards);
        assert!(cards.len() == 5);
        Ok(Hand { cards, bid, htype })
    }
}

#[derive(Debug, Clone)]
struct JokerHand {
    cards: Vec<u8>,
    bid: u32,
    htype: HandType,
}

impl std::cmp::PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &JokerHand) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for JokerHand {
    fn cmp(&self, other: &JokerHand) -> Ordering {
        let res = self.htype.cmp(&other.htype);
        match res {
            Equal => self.cards.cmp(&other.cards),
            x => x,
        }
    }
}

impl std::cmp::PartialEq for JokerHand {
    fn eq(&self, other: &JokerHand) -> bool {
        self.cards == other.cards
    }
}
impl std::cmp::Eq for JokerHand {}

impl std::str::FromStr for JokerHand {
    type Err = ();
    fn from_str(s: &str) -> Result<JokerHand, Self::Err> {
        let mut hand: Hand = s.parse().unwrap();
        for i in 0..hand.cards.len() {
            if hand.cards[i] == 11 {
                hand.cards[i] = 1
            }
        }
        let htype = HandType::determine_joker(&hand.cards);
        Ok(JokerHand {
            cards: hand.cards,
            bid: hand.bid,
            htype,
        })
    }
}

#[test]
fn test_joker() {
    let card1: JokerHand = "JKKK2 1".parse().unwrap();
    let card2: JokerHand = "QQQQ2 1".parse().unwrap();
    assert!(card1 < card2);
    let card: JokerHand = "T55J5 1".parse().unwrap();
    assert_eq!(card.htype, HandType::Four)
}
