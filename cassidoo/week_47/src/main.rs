fn main() {
    println!("Hello, world!");
}

fn max_the_stock(stock_ticks: &[isize]) -> isize {
    let mut max = 0;

    for (idx, tick) in stock_ticks.iter().enumerate() {
        let loop_max = stock_ticks
            .iter()
            .skip(idx)
            .map(|n| n - tick)
            .max()
            .unwrap_or(0);

        max = max.max(loop_max);
    }

    max
}

#[cfg(test)]
mod tests {
    use crate::max_the_stock;

    #[test]
    fn examples() {
        let tests = [(vec![7, 1, 5, 3, 6, 4], 5), (vec![7, 6, 4, 3, 1], 0)];

        for (stock_ticks, expected) in tests {
            let actual = max_the_stock(&stock_ticks);
            assert_eq!(expected, actual, "for case: {:?}", stock_ticks);
        }
    }
}
