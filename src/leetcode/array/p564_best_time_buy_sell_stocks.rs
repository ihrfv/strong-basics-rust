fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;

    let mut prices_iter = prices.into_iter().peekable();

    while let Some(current) = prices_iter.next() {
        if let Some(&next) = prices_iter.peek()
            && next > current
        {
            profit += next - current;
        }
    }
    profit
}

fn max_profit_initial_solution(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }

    let mut profit = 0;
    let mut bought_at: Option<i32> = Option::None;

    let mut prices_iter = prices.into_iter().peekable();

    while let Some(current_val) = prices_iter.next() {
        let next_val: Option<&i32> = prices_iter.peek();
        match bought_at {
            Some(bought_at_price) => {
                if is_time_to_sell(current_val, next_val) {
                    bought_at = Option::None;
                    profit += current_val - bought_at_price;
                }
            }
            None => {
                if should_buy(current_val, next_val) {
                    bought_at = Option::Some(current_val);
                }
            }
        }
    }

    profit
}

fn is_time_to_sell(current_val: i32, next_val: Option<&i32>) -> bool {
    next_val.is_none() || *next_val.unwrap() < current_val
}

fn should_buy(current_val: i32, next_val: Option<&i32>) -> bool {
    next_val.is_some() && *next_val.unwrap() > current_val
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert_eq!(0, max_profit(Vec::new()));
    }

    #[test]
    fn leetcode_example_1() {
        assert_eq!(7, max_profit(vec![7, 1, 5, 3, 6, 4]));
    }

    #[test]
    fn ever_decreasing() {
        assert_eq!(5, max_profit(vec![1, 2, 3, 4, 5, 6]));
    }

    #[test]
    fn ever_increasing() {
        assert_eq!(0, max_profit(vec![6, 5, 4, 3, 2, 1]));
    }
}
