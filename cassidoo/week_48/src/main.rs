fn main() {
    println!("Hello, world!");
}

fn prep_meal(dishes: &Vec<usize>) -> usize {
    let mut duration = 0_usize;

    let mut station_1 = 0_usize;
    let mut station_2 = 0_usize;

    for dish in dishes {
        // First determine how long it will be for the either station 1 or 2 to
        // finish prepping.
        let switch = station_1.min(station_2);

        // Subtract that time from both station 1 and 2, while adding it to the
        // total duration
        station_1 -= switch;
        station_2 -= switch;
        duration += switch;

        // At this point either station 1 or 2 is at 0, meaning they can accept
        // the next dish
        if station_1 == 0 {
            station_1 += dish;
        } else {
            station_2 += dish;
        }
    }

    // No dishes remain, but we need to wait for both station 1 and 2 to finish.
    // So just take the longest and add that to the final duration
    duration += station_1.max(station_2);

    // Return the time as hours, so divide by 60 and round up.
    duration.div_ceil(60)
}

#[cfg(test)]
mod tests {
    use crate::prep_meal;

    #[test]
    fn example_tests() {
        let tests = [
            (vec![120], 2),
            (vec![60, 5, 5, 5, 5], 1),
            (vec![60, 5, 60, 5, 5], 2),
            // # Examples from the original problem
            // (vec![30, 30, 30, 20], 2),
            // (vec![30, 25, 45, 30, 60, 15], 3),
        ];

        for (dishes, expected) in tests {
            let actual = prep_meal(&dishes);
            assert_eq!(actual, expected, "for case: {:?}", dishes);
        }
    }
}
