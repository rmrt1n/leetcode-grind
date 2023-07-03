use std::cmp::Ordering;

pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r) = (0, nums.len() - 1);

    while l <= r {
        let m = l + (r - l) / 2;
        match nums[m].cmp(&target) {
            Ordering::Equal => return m as i32,
            Ordering::Less => l = m + 1,
            Ordering::Greater => r = m - 1,
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = search(vec![-1, 0, 3, 5, 9, 12], 9);
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let res = search(vec![-1, 0, 3, 5, 9, 12], 2);
        assert_eq!(res, -1);
    }
}
