use advent_of_code_2023::*;

fn part1(cards: &[Card]) -> u32 {
    cards.iter().map(|c| c.value()).sum()
}

fn part2(cards: &mut [Card]) -> u32 {
    for i in 0..cards.len() {
        let w = cards[i].matching() as usize;
        for n in (i + 1)..(i + w + 1) {
            cards[n].copies += cards[i].copies;
        }
    }
    cards.iter().map(|c| c.copies).sum()
}

fn main() {
    let mut input: Vec<Card> = read_lines("in/day4");
    println!("Solution 1: {}", part1(&input));
    println!("Solution 2: {}", part2(&mut input));
}

#[test]
fn test() {
    assert_eq!(13, part1(&read_lines("in/day4test")));
    assert_eq!(30, part2(&mut read_lines("in/day4test")));
}

#[derive(Debug, Clone)]
struct Card {
    winning: Vec<u32>,
    nums: Vec<u32>,
    copies: u32,
}

impl Card {
    pub fn matching(&self) -> u32 {
        self.nums
            .iter()
            .filter(|n| self.winning.contains(n))
            .count() as u32
    }
    pub fn value(&self) -> u32 {
        let n = self.matching();
        if n == 0 {
            0
        } else {
            2_u32.pow(n - 1)
        }
    }
}

impl std::str::FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Card, ()> {
        let colon = s.find(':').unwrap();
        let sep = s.find('|').unwrap();
        let winning = s[colon + 1..sep]
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse().unwrap())
            .collect();
        let nums = s[sep + 1..]
            .trim()
            .split(' ')
            .filter(|n| !n.is_empty())
            .map(|n| n.trim().parse().unwrap())
            .collect();
        Ok(Card {
            winning,
            nums,
            copies: 1,
        })
    }
}
