use core::str;

fn main() {
    println!("Hello, world!");
}

fn solve(input: &str, separator: char) -> Vec<String> {
    let mut results: Vec<String> = vec![];

    let mut buf: Vec<char> = vec![];

    for c in input.chars() {
        if c == separator {
            let a: String = buf.iter().collect();
            results.push(a);
            buf = vec![];
        } else {
            buf.push(c);
        }
    }

    if !buf.is_empty() {
        let a: String = buf.iter().collect();
        results.push(a);
    }

    results
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example() {
        let input = "This is so, so silly";
        let tests = vec![
            (' ', vec!["This", "is", "so,", "so", "silly"]),
            (',', vec!["This is so", " so silly"]),
        ];

        for (separator, expected) in tests {
            let actual = solve(input, separator);
            assert_eq!(expected, actual);
        }
    }
}
