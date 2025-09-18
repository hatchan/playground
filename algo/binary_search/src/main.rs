use std::cmp::Ordering;

fn main() {
    let none = &vec![];

    assert_eq!(binary_search(&none, 99), None, "none, target: {}", 99);

    let single = vec![5];

    assert_eq!(binary_search(&single, 5), Some(0), "single, target: {}", 5);
    assert_eq!(binary_search(&single, 99), None, "single, target: {}", 99);

    let multiple = vec![1, 2, 3, 4, 5, 6, 7, 8];

    assert_eq!(binary_search(&multiple, 1), Some(0), "target: {}", 1);
    assert_eq!(binary_search(&multiple, 2), Some(1), "target: {}", 2);
    assert_eq!(binary_search(&multiple, 3), Some(2), "target: {}", 3);
    assert_eq!(binary_search(&multiple, 4), Some(3), "target: {}", 4);
    assert_eq!(binary_search(&multiple, 5), Some(4), "target: {}", 5);
    assert_eq!(binary_search(&multiple, 6), Some(5), "target: {}", 6);
    assert_eq!(binary_search(&multiple, 7), Some(6), "target: {}", 7);
    assert_eq!(binary_search(&multiple, 8), Some(7), "target: {}", 8);

    assert_eq!(binary_search(&multiple, 99), None, "target: {}", 99);

    eprintln!("All tests pass!")
}

/// Given a unique, sorted list of i32, search for target. If found returns the
/// index, none if nothing was found.
fn binary_search(input: &[i32], target: i32) -> Option<usize> {
    let mut lower = 0;
    let mut upper = input.len();
    let mut p;

    loop {
        if lower == upper {
            return None;
        }

        let len = upper - lower;

        // Find the middle in the section
        p = lower + ((len) / 2);

        // Compare this middle element to the target
        let comparison = target.cmp(&input[p]);

        // if it is equal, then we are done. Otherwise check if there is only 1
        // element in this range and stop if it is.
        if comparison == Ordering::Equal {
            return Some(p);
        } else if len == 1 {
            return None;
        }

        // Update the bounds for the next comparison.
        if comparison == Ordering::Less {
            upper = p;
        } else if comparison == Ordering::Greater {
            lower = p + 1;
        }
    }
}
