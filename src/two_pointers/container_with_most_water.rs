use std::cmp::Ordering;

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut res = 0;
    let (mut i, mut j) = (0, height.len() - 1);

    while i < j {
        let area = height[i].min(height[j]) * (j - i) as i32;
        res = res.max(area);
        // for some reason match performs better than regular if/else stmts in lc
        match height[i].cmp(&height[j]) {
            Ordering::Less | Ordering::Equal => i += 1,
            Ordering::Greater => j -= 1,
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]);
        assert_eq!(res, 49);
    }

    #[test]
    fn test2() {
        let res = max_area(vec![1, 1]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test3() {
        let res = max_area(vec![2, 3, 4, 5, 18, 17, 6]);
        assert_eq!(res, 17);
    }
}
