// You are given an array of arrays, where each inner array represents the runs
//. scored by each team in an inning of a baseball game: [[home1, away1], [home2, away2], ...].
//
// Write a function that returns an object with the total runs for each team,
// which innings each team led, and who won the game.

fn main() {
    println!("Hello, world!");
}

fn calculate_stats(innings: Vec<(usize, usize)>) -> Stats {
    innings
        .into_iter()
        .fold(Stats::default(), |mut stats, (home_points, away_points)| {
            stats.home_total += home_points;
            stats.away_total += away_points;

            if home_points >= away_points {
                stats.home_led_innings.push(home_points);
            } else {
                stats.away_led_innings.push(away_points);
            }

            stats.outcome = if stats.home_total == stats.away_total {
                Outcome::Tied
            } else if stats.home_total > stats.away_total {
                Outcome::Home
            } else {
                Outcome::Away
            };

            stats
        })
}

#[derive(Default, PartialEq, Eq, Debug)]
struct Stats {
    home_total: usize,
    away_total: usize,
    home_led_innings: Vec<usize>,
    away_led_innings: Vec<usize>,
    outcome: Outcome,
}

#[derive(Default, PartialEq, Eq, Debug)]
enum Outcome {
    #[default]
    Tied,

    Home,
    Away,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let expected = Stats {
            home_total: 7,
            away_total: 6,
            home_led_innings: vec![1, 2, 4],
            away_led_innings: vec![3],
            outcome: Outcome::Home,
        };

        let innings = vec![(1, 0), (2, 2), (0, 3), (4, 1)];
        let actual = calculate_stats(innings);

        assert_eq!(expected, actual);
    }
}
