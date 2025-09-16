// For an array of numbers, generate an array where for every element, all
// neighboring elements are added to itself, and return the sum of that array.

fn main() {
    println!("Hello, world!");
}

fn sum_neighbors(input: Vec<isize>) -> isize {
    let mut sum = 0;
    for i in 0..input.len() {
        if i != 0 {
            sum += input.get(i - 1).copied().unwrap_or_default();
        }

        sum += input.get(i).copied().unwrap_or_default()
            + input.get(i + 1).copied().unwrap_or_default();
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        struct TestCase {
            input: Vec<isize>,
            expected: isize,
        }

        let tests = vec![
            TestCase {
                input: vec![],
                expected: 0,
            },
            TestCase {
                input: vec![1],
                expected: 1,
            },
            TestCase {
                input: vec![1, 4],
                expected: 10,
            },
            TestCase {
                input: vec![1, 4, 7],
                expected: 28,
            },
            TestCase {
                input: vec![1, 4, 7, 10],
                expected: 55,
            },
            TestCase {
                input: vec![-1, -2, -3],
                expected: -14,
            },
            TestCase {
                input: vec![1, -20, 300, -4000, 50000, -600000, 7000000],
                expected: 12338842,
            },
        ];

        for TestCase { input, expected } in tests {
            let actual = sum_neighbors(input);
            assert_eq!(expected, actual);
        }
    }
}
