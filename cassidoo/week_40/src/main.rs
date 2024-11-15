use core::str;

fn main() {
    println!("Hello, world!");
}

fn solve(input: &str, separator: char) -> Vec<&str> {
    let mut results: Vec<&str> = vec![];

    let mut begin = 0;

    for (i, c) in input.char_indices() {
        if c == separator {
            results.push(&input[begin..i]);
            begin = i + 1;
        }
    }

    if begin != input.len() {
        results.push(&input[begin..]);
    }

    eprintln!("begin: {}; input.len(): {}", begin, input.len());

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
            ('y', vec!["This is so, so sill"]),
        ];

        for (separator, expected) in tests {
            let actual = solve(input, separator);
            assert_eq!(expected, actual);
        }
    }
}
