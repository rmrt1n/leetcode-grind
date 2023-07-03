pub fn nc_max_profit(prices: Vec<i32>) -> i32 {
    let (mut l, mut r) = (0, 1);
    let mut max = 0;

    while r < prices.len() {
        if prices[l] > prices[r] {
            l = r;
            r += 1;
            continue;
        }
        max = max.max(prices[r] - prices[l]);
        r += 1;
    }

    max
}

pub fn max_profit(prices: Vec<i32>) -> i32 {
    prices
        .into_iter()
        .fold((0, i32::MAX), |(max, buy), i| {
            (max.max(i - buy), buy.min(i))
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_test1() {
        let res = nc_max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(res, 5);
    }

    #[test]
    fn nc_test2() {
        let res = nc_max_profit(vec![7, 6, 4, 3, 1]);
        assert_eq!(res, 0);
    }

    #[test]
    fn test1() {
        let res = max_profit(vec![7, 1, 5, 3, 6, 4]);
        assert_eq!(res, 5);
    }

    #[test]
    fn test2() {
        let res = max_profit(vec![7, 6, 4, 3, 1]);
        assert_eq!(res, 0);
    }
}
