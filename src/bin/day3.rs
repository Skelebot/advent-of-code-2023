use advent_of_code_2023::*;
use std::str::FromStr;

fn part1(lines: &[Line]) -> u32 {
    let mut sum = 0;
    for i in 0..lines.len() {
        let prev = if i > 0 { lines.get(i - 1) } else { None };
        let next = lines.get(i + 1);
        let n = lines[i].sum(prev, next);
        sum += n;
    }
    sum
}

fn part2(lines: &[Line]) -> u32 {
    let mut sum = 0;
    for i in 0..lines.len() {
        let prev = if i > 0 { lines.get(i - 1) } else { None };
        let next = lines.get(i + 1);
        let n = lines[i].gears(prev, next);
        sum += n;
    }
    sum
}

fn main() {
    let input: Vec<Line> = read_lines("in/day3");
    println!("Solution 1: {}", part1(&input));
    println!("Solution 2: {}", part2(&input));
}

#[test]
fn test() {
    assert_eq!(4361, part1(&read_lines("in/day3test")));
    assert_eq!(467835, part2(&read_lines("in/day3test")));
}

#[derive(Debug, Copy, Clone)]
struct Num {
    pub val: u32,
    // (start, end inclusive)
    pub extents: (usize, usize),
}

// (index, is a star)
#[derive(Debug, Copy, Clone)]
struct Sym(usize, bool);

#[derive(Debug)]
struct Line(Vec<Num>, Vec<Sym>);

impl Line {
    pub fn sum(&self, prev: Option<&Line>, next: Option<&Line>) -> u32 {
        let mut sum = 0;
        let mut symbols: Vec<Sym> = Vec::new();
        symbols.extend(&self.1);
        if let Some(prev) = prev {
            symbols.extend(&prev.1)
        }
        if let Some(next) = next {
            symbols.extend(&next.1)
        }
        'nums: for num in self.0.iter() {
            for sym in symbols.iter() {
                if Line::near(num.extents, &sym.0) {
                    sum += num.val;
                    continue 'nums;
                }
            }
        }
        sum
    }
    pub fn gears(&self, prev: Option<&Line>, next: Option<&Line>) -> u32 {
        let mut sum = 0;
        let mut nums: Vec<Num> = Vec::new();
        nums.extend(&self.0);
        if let Some(prev) = prev {
            nums.extend(&prev.0)
        }
        if let Some(next) = next {
            nums.extend(&next.0)
        }
        // only star symbols
        for star in self.1.iter().filter(|s| s.1) {
            let mut adj: Vec<&Num> = Vec::with_capacity(2);
            for num in nums.iter() {
                if Line::near(num.extents, &star.0) {
                    adj.push(num);
                }
            }
            if adj.len() == 2 {
                sum += adj[0].val * adj[1].val;
            }
        }
        sum
    }
    pub fn near(extents: (usize, usize), pos: &usize) -> bool {
        if pos + 1 == extents.0 || pos - 1 == extents.1 || (extents.0..=extents.1).contains(pos) {
            return true;
        }
        false
    }
}

impl FromStr for Line {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();
        let mut nums = Vec::new();
        let mut syms = Vec::new();
        let mut idx = 0;
        while idx < s.len() {
            match s.chars().nth(idx).unwrap() {
                '.' => (),
                c if c.is_ascii_digit() => {
                    let end = s[idx..]
                        .find(|c: char| !c.is_ascii_digit())
                        .unwrap_or(s.len() - idx);
                    let num = s[idx..idx + end].parse::<u32>().unwrap();
                    nums.push(Num {
                        val: num,
                        extents: (idx, idx + end - 1),
                    });
                    idx += end - 1;
                }
                c if c.is_ascii_punctuation() => {
                    syms.push(Sym(idx, c == '*'));
                }
                _ => panic!("Invalid char"),
            }
            idx += 1;
        }
        for subs in s.split('.') {
            match subs.trim() {
                x if x.is_empty() => continue,
                "\n" => continue,
                x if x.parse::<u32>().is_ok() => (),
                _ => {}
            }
        }
        Ok(Line(nums, syms))
    }
}
