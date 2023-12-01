use advent_of_code_2023::*;

fn part1(lines: &[String]) -> u32 {
    let mut sum: u32 = 0;
    for l in lines {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        for c in l.chars() {
            if c.is_numeric() {
                let b = c as u32 - b'0' as u32;
                if first.is_none() { first = Some(b) }
                last = Some(b)
            }
        }
        sum += first.unwrap() * 10 + last.unwrap()
    }
    return sum;
}

fn part2(lines: &[String]) -> u32 {
    let nums = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum: u32 = 0;
    for l in lines {
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        let mut start = 0;
        while start < l.len() {
            let c = l.chars().nth(start).unwrap();
            if c.is_numeric() {
                let b = c as u32 - b'0' as u32;
                if first.is_none() { first = Some(b) }
                last = Some(b);
            } else {
                'inner: for (i, n) in nums.iter().enumerate() {
                    let left = l.len() - start;
                    if left >= n.len() && &l[start..start + n.len()] == *n {
                        if first.is_none() { first = Some(i as u32 + 1) }
                        last = Some(i as u32 + 1);
                        break 'inner;
                    }
                }
            }
            start += 1;
        }
        println!("{}, {:?}, {:?}", l, first, last);
        sum += first.unwrap() * 10 + last.unwrap()
    }
    return sum;
}

fn main() {
    let input: Vec<String> = read_lines("in/day1");
    println!("Solution 1: {}", part1(&input));
    println!("Solution 2: {}", part2(&input));
}

#[test]
fn test() {
    assert_eq!(142, part1(&read_lines("in/day1test")));
    assert_eq!(281, part2(&read_lines("in/day1test2")));
}
