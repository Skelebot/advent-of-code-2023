use advent_of_code_2023::*;
use num_integer::lcm;
use std::collections::HashMap;

fn part1(inp: &(&str, HashMap<&str, (&str, &str)>)) -> u64 {
    steps_until(inp, "AAA", |s| s == "ZZZ")
}

fn part2(inp: &(&str, HashMap<&str, (&str, &str)>)) -> u64 {
    inp.1
        .keys()
        .filter(|k| k.ends_with('A'))
        .map(|k| steps_until(inp, k, |f| f.ends_with('Z')))
        .reduce(|acc, stps| lcm(acc, stps))
        .unwrap()
}

fn steps_until<F>(inp: &(&str, HashMap<&str, (&str, &str)>), start: &str, target: F) -> u64
where
    F: Fn(&str) -> bool,
{
    let (instrs, map) = inp;
    let mut steps = 0;
    let mut curr: &str = start;
    let mut instr_id = 0;
    while !target(curr) {
        match instrs.chars().nth(instr_id) {
            Some('L') => curr = map[curr].0,
            Some('R') => curr = map[curr].1,
            _ => panic!("invalid instruction"),
        }
        steps += 1;
        instr_id = (instr_id + 1) % instrs.len()
    }
    steps
}

fn main() {
    let input: Vec<String> = read_lines("in/day8");
    let inp = read_input(&input);
    println!("Solution 1: {}", part1(&inp));
    println!("Solution 2: {}", part2(&inp));
}

#[test]
fn test() {
    assert_eq!(6, part1(&read_input(&read_lines("in/day8test"))));
    assert_eq!(6, part2(&read_input(&read_lines("in/day8test2"))));
}

fn read_input<'a>(lines: &'a [String]) -> (&'a str, HashMap<&'a str, (&'a str, &'a str)>) {
    (
        lines[0].trim(),
        lines[2..]
            .iter()
            .map(|l| (&l[0..3], (&l[7..10], &l[12..15])))
            .collect(),
    )
}
