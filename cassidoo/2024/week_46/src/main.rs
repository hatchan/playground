fn main() {
    println!("Hello, world!");
}

fn solve(input: Vec<usize>) -> usize {
    #[derive(Default)]
    struct acc {
        max: usize,
        len: usize,
    };
    let acc { len, .. } = input.iter().fold(acc::default(), |mut acc, &x| {
        if x > acc.max {
            acc.max = x;
            acc.len += 1;
        }

        acc
    });

    len
}

#[cfg(test)]
mod tests {
    use crate::solve;

    #[test]
    fn example() {
        let tests = vec![
            (5, vec![1, 2, 3, 4, 5]),
            (1, vec![5, 4, 3, 2, 1]),
            (3, vec![3, 7, 8, 3, 6, 1]),
            (4, vec![1, 2, 1, 1, 1, 4, 3, 7]),
        ];

        for (expected, input) in tests {
            let actual = solve(input);
            assert_eq!(expected, actual);
        }
    }
}
