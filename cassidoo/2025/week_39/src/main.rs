// Write a function that determines if a number is abundant, deficient, perfect,
// or amicable. See: https://www.encyclopedia.com/education/news-wires-white-papers-and-books/numbers-abundant-deficient-perfect-and-amicable

use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
}

fn kind_of_number(target: &i32) -> KindOfNumber {
    // take the numbers from 1 until half of the target and see if target can be
    // divided by them. Take those numbers and sum them all up
    let sum: i32 = (1..=(target / 2)).filter(|i| target % i == 0).sum();

    // If the sum if equal to the target then it is considered a perfect number,
    // if it is less, then it is Deficient, and greater than it is Abundant.
    match sum.cmp(target) {
        Ordering::Less => KindOfNumber::Deficient,
        Ordering::Equal => KindOfNumber::Perfect,
        Ordering::Greater => KindOfNumber::Abundant,
    }
}

#[derive(Debug, PartialEq)]
enum KindOfNumber {
    Abundant,
    Deficient,
    Perfect,
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn kind_of_number_tests() {
        let abundant_number = 12;
        assert_eq!(
            kind_of_number(&abundant_number),
            KindOfNumber::Abundant,
            "Should be abundant: {abundant_number}"
        );

        let deficient_number = 4;
        assert_eq!(
            kind_of_number(&deficient_number),
            KindOfNumber::Deficient,
            "Should be deficient: {deficient_number}"
        );

        let perfect_number = 6;
        assert_eq!(
            kind_of_number(&perfect_number),
            KindOfNumber::Perfect,
            "Should be perfect: {perfect_number}"
        );
    }
}
