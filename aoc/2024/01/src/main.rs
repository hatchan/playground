const INPUT: &'static str = include_str!("../input.txt");

fn main() {
    println!("Day 1a: {}", solve_a(INPUT));
}

fn solve_a(input: &str) -> usize {
    let (mut left, mut right) = parse(input);

    left.sort_unstable();
    right.sort_unstable();

    let result = left
        .into_iter()
        .zip(right)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    result
}

fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
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

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = include_str!("../example.txt");

    #[test]
    fn example_a() {
        let result = solve_a(EXAMPLE_INPUT);
        assert_eq!(result, 11);
    }
}
