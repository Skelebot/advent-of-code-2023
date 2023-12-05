use advent_of_code_2023::*;
use std::ops::Range;

fn part1(almanac: &Almanac) -> usize {
    let mut min = usize::MAX;
    for seed in almanac.seeds.iter() {
        let loc = almanac.map_through_all(*seed);
        if loc < min {
            min = loc
        };
    }
    min
}

fn part2(almanac: &Almanac) -> usize {
    let mut seeds: Vec<Range<usize>> = Vec::new();
    for i in (0..almanac.seeds.len()).step_by(2) {
        let range = almanac.seeds[i]..(almanac.seeds[i] + almanac.seeds[i + 1]);
        seeds.push(range);
    }
    let mut min = usize::MAX;
    for range in seeds {
        for seed in range {
            let loc = almanac.map_through_all(seed);
            if loc < min {
                min = loc
            };
        }
    }
    min
}

fn main() {
    let input: Almanac = read_parse("in/day5");
    println!("Solution 1: {}", part1(&input));
    println!("Solution 2: {}", part2(&input));
}

#[test]
fn test() {
    assert_eq!(35, part1(&read_parse("in/day5test")));
    assert_eq!(46, part2(&read_parse("in/day5test")));
}

#[derive(Debug)]
struct Mapping {
    pub src: Range<usize>,
    pub dest: Range<usize>,
}

impl Mapping {
    pub fn map(&self, s: usize) -> usize {
        if self.src.contains(&s) {
            self.dest.start + (s - self.src.start)
        } else {
            s
        }
    }
    pub fn contains(&self, s: usize) -> bool {
        self.src.contains(&s)
    }
}

struct MappingSet {
    mappings: Vec<Mapping>,
}

impl MappingSet {
    pub fn map(&self, s: usize) -> usize {
        for set in self.mappings.iter() {
            if set.contains(s) {
                return set.map(s);
            }
        }
        s
    }
}

struct Almanac {
    seeds: Vec<usize>,
    sets: Vec<MappingSet>,
}

impl Almanac {
    pub fn map_through_all(&self, s: usize) -> usize {
        let mut cur = s;
        for set in self.sets.iter() {
            cur = set.map(cur);
        }
        cur
    }
}

impl std::str::FromStr for Almanac {
    type Err = ();
    fn from_str(s: &str) -> Result<Almanac, ()> {
        let mut lines = s.lines();
        let seeds: Vec<usize> = lines
            .next()
            .unwrap()
            .trim()
            .split(' ')
            .skip(1)
            .map(|n| n.parse().unwrap())
            .collect();

        let mut sets = Vec::new();
        let mut curset = MappingSet {
            mappings: Vec::new(),
        };
        let mut lines2 = lines.skip(2);
        loop {
            match lines2.next() {
                Some(l) if l.is_empty() => {
                    let _ = lines2.next(); // skip text
                    sets.push(curset); // new set begins
                    curset = MappingSet {
                        mappings: Vec::new(),
                    };
                }
                Some(l) if l.trim().chars().next().unwrap().is_ascii_digit() => {
                    let nums: Vec<usize> = l.split(' ').map(|n| n.parse().unwrap()).collect(); // should have 3 numbers
                    curset.mappings.push(Mapping {
                        src: nums[1]..nums[1] + nums[2],
                        dest: nums[0]..nums[0] + nums[2],
                    });
                }
                None => {
                    sets.push(curset); // new set begins
                    break;
                }
                _ => panic!("Invalid data"),
            }
        }
        Ok(Almanac { seeds, sets })
    }
}
