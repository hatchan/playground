// Given an array arr representing the positions of monsters along a straight
// line, and an integer d representing the minimum safe distance required
// between any two monsters, write a function to determine if all monsters are
// at least d units apart. If not, return the smallest distance found between
// any two monsters. If all monsters are safely spaced, return -1.

fn main() {
    println!("Hello, world!");
}

fn check_if_safe(monsters: Vec<usize>, safe_distance: usize) -> Safety {
    let mut safety = Safety::Safe;

    for i in 0..monsters.len() {
        if let Some(next) = monsters.get(i + 1) {
            let diff = next - monsters[i];
            if diff < safe_distance {
                match safety {
                    Safety::Safe => safety = Safety::NotSafe(diff),
                    Safety::NotSafe(prev) if prev > diff => safety = Safety::NotSafe(diff),
                    _ => (),
                };
            }
        }
    }

    safety
}

#[derive(Debug, PartialEq, Eq)]
enum Safety {
    Safe,
    NotSafe(usize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        struct TestCase {
            monsters: Vec<usize>,
            safe_distance: usize,
            expected: Safety,
        }

        let tests = vec![
            TestCase {
                monsters: vec![3, 8, 10, 15],
                safe_distance: 6,
                expected: Safety::NotSafe(2),
            },
            TestCase {
                monsters: vec![5, 9, 14, 18],
                safe_distance: 4,
                expected: Safety::Safe,
            },
        ];

        for TestCase {
            monsters,
            safe_distance,
            expected,
        } in tests
        {
            let actual = check_if_safe(monsters, safe_distance);
            assert_eq!(expected, actual);
        }
    }
}
