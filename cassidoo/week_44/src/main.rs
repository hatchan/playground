use rand::Rng;
use std::fmt::{Debug, Display};

#[derive(Debug)]
pub struct DiceThrow([u8; 5]);

impl DiceThrow {
    pub fn throw() -> Self {
        Self([
            roll_dice(),
            roll_dice(),
            roll_dice(),
            roll_dice(),
            roll_dice(),
        ])
    }

    pub fn total(&self) -> u8 {
        self.0.iter().sum()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, u8> {
        self.0.iter()
    }
}

impl Display for DiceThrow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Clone for DiceThrow {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl Copy for DiceThrow {}

#[derive(Debug)]
enum Options {
    Unit { number: u8, count: u8 },
    Chance(DiceThrow),
    ThreeOfAKind(DiceThrow),
    FourOfAKind(DiceThrow),
    FullHouse,
    SmallStraight,
    LargeStraight,
    Yahtzee,
}

impl Display for Options {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Options::Unit { number, count } => {
                f.write_fmt(format_args!("Unit {} ({}x)", number, count))
            }
            Options::Chance(numbers) => f.write_fmt(format_args!("Chance {}", numbers)),
            Options::ThreeOfAKind(_) => f.write_str("Three of a kind"),
            Options::FourOfAKind(_) => f.write_str("Four of a kind"),
            Options::FullHouse { .. } => f.write_str("Full house"),
            Options::SmallStraight => f.write_str("Small straight"),
            Options::LargeStraight => f.write_str("Large straight"),
            Options::Yahtzee => f.write_str("Yahtzee"),
        }
    }
}

impl Options {
    pub fn score(&self) -> u8 {
        match self {
            Options::Unit { number, count } => number * count,
            Options::Chance(numbers) => numbers.total(),
            Options::ThreeOfAKind(numbers) => numbers.total(),
            Options::FourOfAKind(numbers) => numbers.total(),
            Options::FullHouse { .. } => 25,
            Options::SmallStraight => 30,
            Options::LargeStraight => 40,
            Options::Yahtzee => 50,
        }
    }
}

fn main() {
    let numbers = DiceThrow::throw();

    // let numbers = [5, 4, 2, 3, 6];

    let options = calc_options(&numbers);

    println!("Numbers: {:?}", numbers);
    println!("Options:");
    for option in &options {
        println!("- {} score: {}", option, option.score());
    }
}

fn calc_options(numbers: &DiceThrow) -> Vec<Options> {
    let mut options = vec![Options::Chance(*numbers)];

    let grouped = numbers.iter().fold([0; 6], |mut acc, num| {
        acc[*num as usize - 1] += 1;
        acc
    });

    // units
    options.extend(
        grouped
            .iter()
            .enumerate()
            .filter(|(_, count)| **count > 0)
            .map(|(number, count)| Options::Unit {
                number: (number + 1) as u8,
                count: *count as u8,
            }),
    );

    // Three of a kind, Four of a kind, Yahtzee
    grouped.iter().for_each(|count| {
        if *count >= 3 {
            options.push(Options::ThreeOfAKind(*numbers))
        }

        if *count >= 4 {
            options.push(Options::FourOfAKind(*numbers))
        }

        if *count >= 5 {
            options.push(Options::Yahtzee)
        }
    });

    // Full house
    let is_full_house = grouped
        .iter()
        .find(|counts| {
            if **counts == 3 {
                grouped.iter().find(|counts| **counts == 2).is_some()
            } else {
                false
            }
        })
        .is_some();

    if is_full_house {
        options.push(Options::FullHouse);
    }

    // Large straight
    let is_large_straight = grouped[0..=4].iter().filter(|count| **count > 0).count() == 5
        || grouped[1..=5].iter().filter(|count| **count > 0).count() == 5;

    if is_large_straight {
        options.push(Options::LargeStraight);
    }
    // Small straight (if it is a lane straight, then it also is a small straight)
    let is_small_straight = is_large_straight
        || grouped[0..=3].iter().filter(|count| **count > 0).count() == 4
        || grouped[1..=4].iter().filter(|count| **count > 0).count() == 4
        || grouped[2..=5].iter().filter(|count| **count > 0).count() == 4;

    if is_small_straight {
        options.push(Options::SmallStraight);
    }

    options
}

fn roll_dice() -> u8 {
    let mut rng = rand::thread_rng();
    rng.gen_range(1..=6)
}
