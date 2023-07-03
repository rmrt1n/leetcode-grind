pub fn nc_find_min(nums: Vec<i32>) -> i32 {
    let (mut l, mut r): (usize, i32) = (0, nums.len() as i32 - 1);
    let mut res = nums[0];

    while (l as i32) < r {
        let m = l + (r as usize - l) / 2;
        res = res.min(nums[m]);

        if nums[m] > nums[r as usize] {
            l = m + 1;
        } else {
            r = m as i32 - 1;
        }
    }

    res.min(nums[l])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_test1() {
        let res = nc_find_min(vec![3, 4, 5, 1, 2]);
        assert_eq!(res, 1);
    }

    #[test]
    fn nc_test2() {
        let res = nc_find_min(vec![4, 5, 6, 7, 0, 1, 2]);
        assert_eq!(res, 0);
    }

    #[test]
    fn nc_test3() {
        let res = nc_find_min(vec![11, 13, 15, 17]);
        assert_eq!(res, 11);
    }

    #[test]
    fn nc_test4() {
        let res = nc_find_min(vec![1, 2]);
        assert_eq!(res, 1);
    }

    #[test]
    fn nc_test5() {
        let res = nc_find_min(vec![2, 3, 1]);
        assert_eq!(res, 1);
    }

    #[test]
    fn nc_test6() {
        let res = nc_find_min(vec![3, 1, 2]);
        assert_eq!(res, 1);
    }

    #[test]
    fn nc_test7() {
        let res = nc_find_min(vec![5, 1, 2, 3, 4]);
        assert_eq!(res, 1);
    }
}
