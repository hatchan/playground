const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Day 2a: {}", solve_a(INPUT));
    println!("Day 2b: {}", solve_b(INPUT));
}

fn solve_a(input: &str) -> usize {
    let input = parse_input(input);

    input
        .iter()
        .filter(|report| is_gradual(report.iter()))
        .count()
}

fn is_gradual<'a, T>(report: T) -> bool
where
    T: Iterator<Item = &'a isize>,
{
    let mut previous_diff: Option<isize> = None;
    let mut previous_level: Option<&isize> = None;

    for current_level in report {
        let current_diff = match previous_level {
            Some(prev_level) => prev_level - current_level,
            None => {
                previous_level = Some(current_level);
                continue;
            }
        };

        // Make sure that the difference is 1,2,3
        let current_diff_abs = current_diff.abs();
        if !(current_diff_abs > 0 && current_diff_abs <= 3) {
            return false;
        }

        // Make sure that the sign has not changed
        if let Some(prev_diff) = previous_diff {
            if (current_diff < 0) != (prev_diff < 0) {
                return false;
            }
        }

        previous_diff = Some(current_diff);
        previous_level = Some(current_level);
    }

    true
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
                let input =
                    a.iter()
                        .enumerate()
                        .filter_map(|(idx, val)| if idx == i { None } else { Some(val) });
                if is_gradual(input) {
                    return true;
                }
            }

            false
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
