use std::str::FromStr;

use advent_of_code_2023::*;

fn part1(games: &[Game]) -> u32 {
    games
        .iter()
        .filter(|g| g.possible(12, 13, 14))
        .map(|g| g.id as u32)
        .sum()
}

fn part2(games: &[Game]) -> u32 {
    games.iter().map(|g| g.minimum_power()).sum()
}

fn main() {
    let input: Vec<Game> = read_lines("in/day2");
    println!("Solution 1: {}", part1(&input));
    println!("Solution 2: {}", part2(&input));
}

struct Game {
    pub id: usize,
    pub sets: Vec<(u32, u32, u32)>,
}

impl Game {
    pub fn possible(&self, r_max: u32, g_max: u32, b_max: u32) -> bool {
        for set in self.sets.iter() {
            if set.0 > r_max {
                return false;
            }
            if set.1 > g_max {
                return false;
            }
            if set.2 > b_max {
                return false;
            }
        }
        true
    }
    pub fn minimum_power(&self) -> u32 {
        Game::power(self.minimum_set())
    }
    pub fn minimum_set(&self) -> (u32, u32, u32) {
        (
            self.sets.iter().map(|s| s.0).max().unwrap(),
            self.sets.iter().map(|s| s.1).max().unwrap(),
            self.sets.iter().map(|s| s.2).max().unwrap(),
        )
    }
    fn power(set: (u32, u32, u32)) -> u32 {
        set.0 * set.1 * set.2
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let semic = s.find(':').unwrap();
        let id: usize = (s[5..semic]).parse().unwrap();
        let mut sets = Vec::new();
        for sub_set in s[semic + 2..].split(';').map(str::trim) {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            for sub_color in sub_set.split(',').map(str::trim) {
                let space = sub_color.find(' ').unwrap();
                let num: u32 = sub_color[0..space].parse().unwrap();
                match &sub_color[space + 1..] {
                    "red" => red += num,
                    "green" => green += num,
                    "blue" => blue += num,
                    _ => panic!("Invalid input set"),
                }
            }
            sets.push((red, green, blue));
        }
        Ok(Game { id, sets })
    }
}

#[test]
fn test_game_from_str() {
    let game = Game::from_str("Game 34: 5 blue, 4 red, 1 green; 2 green, 6 blue; 1 green").unwrap();
    assert_eq!(game.id, 34);
    assert_eq!(game.sets.len(), 3);
    assert_eq!(game.sets.get(0), Some(&(4, 1, 5)));
    assert_eq!(game.sets.get(1), Some(&(0, 2, 6)));
    assert_eq!(game.sets.get(2), Some(&(0, 1, 0)));
}

#[test]
fn day2_test() {
    assert_eq!(8, part1(&read_lines("in/day2test")));
    assert_eq!(2286, part2(&read_lines("in/day2test")));
}
