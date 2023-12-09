use advent_of_code_2023::*;

fn part1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|l| l.last().unwrap() + get_prediction(l, true))
        .sum()
}

fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|l| l.first().unwrap() - get_prediction(l, false))
        .sum()
}

fn get_prediction(line: &[i64], right: bool) -> i64 {
    let diffs: Vec<i64> = (0..(line.len() - 1))
        .map(|i| line[i + 1] - line[i])
        .collect();
    if diffs.iter().all(|n| *n == 0) {
        0
    } else {
        let pred = get_prediction(&diffs, right);
        if right {
            diffs.last().unwrap() + pred
        } else {
            diffs.first().unwrap() - pred
        }
    }
}

fn main() {
    let input: Vec<Vec<i64>> = read_lines_split("in/day9", " ");
    println!("Solution 1: {}", part1(&input));
    println!("Solution 2: {}", part2(&input));
}

#[test]
fn test() {
    assert_eq!(28, 21 + get_prediction(&[1, 3, 6, 10, 15, 21], true));
    assert_eq!(5, 10 - get_prediction(&[10, 13, 16, 21, 30, 45], false));

    assert_eq!(114, part1(&read_lines_split("in/day9test", " ")));
    assert_eq!(2, part2(&read_lines_split("in/day9test", " ")));
}
