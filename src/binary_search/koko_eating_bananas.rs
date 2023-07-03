use std::cmp::Ordering;

pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
    let (mut l, mut r) = (1, *piles.iter().max().unwrap());

    while l <= r {
        let k = l + (r - l) / 2;
        let time = piles
            .iter()
            .map(|i| (*i as f64 / k as f64).ceil())
            .fold(0, |acc, i| acc + (i as i64));

        match time.cmp(&(h as i64)) {
            Ordering::Equal | Ordering::Less => r = k - 1,
            Ordering::Greater => l = k + 1,
        }
    }

    l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = min_eating_speed(vec![3, 6, 7, 11], 8);
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let res = min_eating_speed(vec![30, 11, 23, 4, 20], 5);
        assert_eq!(res, 30);
    }

    #[test]
    fn test3() {
        let res = min_eating_speed(vec![30, 11, 23, 4, 20], 6);
        assert_eq!(res, 23);
    }

    #[test]
    fn test4() {
        let res = min_eating_speed(vec![805306368, 805306368, 805306368], 1000000000);
        assert_eq!(res, 3);
    }
}
