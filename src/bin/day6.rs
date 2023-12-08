use advent_of_code_2023::*;

fn part1(times: &[u32], distances: &[u32]) -> u128 {
    let mut mul = 1;
    for (t, d) in times.iter().zip(distances.iter()) {
        let ways = ways(*t as u128, *d as u128);
        mul *= ways;
    }
    mul
}

fn part2(time: &u128, distance: &u128) -> u128 {
    ways(*time, *distance)
}

fn ways(t: u128, d: u128) -> u128 {
    let delta = (t * t) - (4 * d);
    let delta_sqrt = (delta as f64).sqrt();
    let x1 = ((-(t as f64)) - delta_sqrt) / (-2f64);
    let x2 = ((-(t as f64)) + delta_sqrt) / (-2f64);
    let mut res = (x1.ceil() - x2.ceil()) as u128;
    if x1.fract().abs() < f64::EPSILON || x2.fract().abs() < f64::EPSILON {
        res -= 1;
    }
    res
}

fn main() {
    let input: Vec<String> = read_lines("in/day6");
    let times = read_numbers(&input[0]);
    let distances = read_numbers(&input[1]);
    println!("Solution 1: {}", part1(&times, &distances));
    let time = read_number(&input[0]);
    let distance = read_number(&input[1]);
    println!("Solution 2: {}", part2(&time, &distance));
}

#[test]
fn test() {
    let input: Vec<String> = read_lines("in/day6test");
    let times = read_numbers(&input[0]);
    let distances = read_numbers(&input[1]);
    assert_eq!(288, part1(&times, &distances));
    let time = read_number(&input[0]);
    let distance = read_number(&input[1]);
    assert_eq!(71503, part2(&time, &distance));
}

fn read_numbers(l: &str) -> Vec<u32> {
    l.split(' ')
        .filter_map(|n| n.trim().parse::<u32>().ok())
        .collect()
}
fn read_number(l: &str) -> u128 {
    let s: String = l.chars().filter(|c| c.is_ascii_digit()).collect();
    s.parse().unwrap()
}
