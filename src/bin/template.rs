use advent_of_code_2023::*;

fn part1(_lines: &[String]) -> u32 {
    0
}

fn part2(_lines: &[String]) -> u32 {
    0
}

fn main() {
    let input: Vec<String> = read_lines("in/dayX");
    println!("Solution 1: {}", part1(&input));
    println!("Solution 2: {}", part2(&input));
}

#[test]
fn test() {
    assert_eq!(0, part1(&read_lines("in/dayXtest")));
    assert_eq!(0, part2(&read_lines("in/dayXtest2")));
}
