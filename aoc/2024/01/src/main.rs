use std::collections::HashMap;

const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("Day 1a: {}", solve_a(INPUT));
    println!("Day 1b: {}", solve_b(INPUT));
}

fn solve_a(input: &str) -> usize {
    let (mut left, mut right) = parse_a(input);

    left.sort_unstable();
    right.sort_unstable();

    let result = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    result
}

fn parse_a(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut left_result = Vec::new();
    let mut right_result = Vec::new();

    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .for_each(|(left, right)| {
            left_result.push(left.parse().unwrap());
            right_result.push(right.parse().unwrap())
        });

    (left_result, right_result)
}

fn solve_b(input: &str) -> usize {
    let (left, right) = parse_b(input);

    let result = left
        .into_iter()
        .map(|left| left * right.get(&left).copied().unwrap_or_default())
        .sum();

    result
}

fn parse_b(input: &str) -> (Vec<usize>, HashMap<usize, usize>) {
    let (left, right) = parse_a(input);

    let right_totals: HashMap<usize, usize> =
        right.into_iter().fold(HashMap::new(), |mut acc, x| {
            acc.entry(x).and_modify(|entry| *entry += 1).or_insert(1);
            acc
        });

    (left, right_totals)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn example_a() {
        let result = solve_a(EXAMPLE_INPUT);
        assert_eq!(result, 11);
    }

    #[test]
    fn example_b() {
        let result = solve_b(EXAMPLE_INPUT);
        assert_eq!(result, 31);
    }
}
