pub fn nc_find_duplicate(nums: Vec<i32>) -> i32 {
    let (mut slow, mut fast) = (nums[0], nums[nums[0] as usize]);
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
    }

    slow = 0;
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }

    slow
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_test1() {
        let res = nc_find_duplicate(vec![1, 3, 4, 2, 2]);
        assert_eq!(res, 2);
    }

    #[test]
    fn nc_test2() {
        let res = nc_find_duplicate(vec![3, 1, 3, 4, 2]);
        assert_eq!(res, 3);
    }

    #[test]
    fn nc_test3() {
        let res = nc_find_duplicate(vec![2, 2, 2, 2, 2]);
        assert_eq!(res, 2);
    }
}
