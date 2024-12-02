const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Day 2a: {}", solve_a(INPUT));
    println!("Day 2b: {}", solve_b(INPUT));
}

fn solve_a(input: &str) -> usize {
    let input = parse_input(input);

    input.into_iter().filter(is_gradual).count()
}

fn is_gradual(input: &Vec<isize>) -> bool {
    let diffs: Vec<_> = input.windows(2).map(|a| a[1] - a[0]).collect();

    let sign_all_same = diffs.iter().all(|&diff| diff > 0) || diffs.iter().all(|&diff| diff < 0);

    if !sign_all_same {
        return false;
    }

    diffs.iter().all(|&diff| diff.abs() > 0 && diff.abs() <= 3)
}

fn parse_input(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|line| line.split(' ').map(|n| n.parse().unwrap()).collect())
        .collect()
}

fn solve_b(input: &str) -> usize {
    let input = parse_input(input);

    input
        .into_iter()
        .filter(|a| {
            for i in 0..a.len() {
                let input: Vec<_> = a
                    .iter()
                    .enumerate()
                    .filter_map(|(idx, val)| if idx == i { None } else { Some(val) })
                    .copied()
                    .collect();
                if is_gradual(&input) {
                    return true;
                }
            }

            return false;
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn example_a() {
        let result = solve_a(EXAMPLE_INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn example_b() {
        let result = solve_b(EXAMPLE_INPUT);
        assert_eq!(result, 4);
    }
}
