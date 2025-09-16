use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

fn solve(input: Vec<&str>) -> Vec<Vec<&str>> {
    let mut result: HashMap<Vec<char>, Vec<&str>> = HashMap::new();

    for word in input {
        let mut chars: Vec<_> = word.chars().collect();
        chars.sort_unstable();
        result.entry(chars).or_default().push(word);
    }

    result.into_values().collect()
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example() {
        let tests = vec![
            (
                vec!["eat", "tea", "tan", "ate", "nat", "bat"],
                vec![vec!["eat", "tea", "ate"], vec!["tan", "nat"], vec!["bat"]],
            ),
            (vec!["vote", "please"], vec![vec!["vote"], vec!["please"]]),
            (
                vec!["debitcard", "badcredit"],
                vec![vec!["debitcard", "badcredit"]],
            ),
        ];

        for (input, expected) in tests {
            let actual = solve(input);
            assert_eq!(actual.len(), expected.len());
            for ex in expected {
                assert!(actual.contains(&ex));
            }
        }
    }
}
