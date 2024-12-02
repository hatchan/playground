const INPUT: &str = include_str!("../input.txt");

fn main() {
    println!("Day 2a: {}", solve_a(INPUT));
    // println!("Day 2b: {}", solve_b(INPUT));
}

fn solve_a(input: &str) -> usize {
    todo!()
}

fn solve_b(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = include_str!("../example.txt");

    #[test]
    fn example_a() {
        let result = solve_a(EXAMPLE_INPUT);
        assert_eq!(result, 11);
    }

    // #[test]
    // fn example_b() {
    //     let result = solve_b(EXAMPLE_INPUT);
    //     assert_eq!(result, 31);
    // }
}
