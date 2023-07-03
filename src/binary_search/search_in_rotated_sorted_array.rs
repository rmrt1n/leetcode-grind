pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut l, mut r): (usize, i32) = (0, nums.len() as i32 - 1);

    while l as i32 <= r {
        let m = l + (r as usize - l) / 2;
        if nums[m] == target {
            return m as i32;
        }
        if nums[l] <= nums[m] {
            // m in left array
            if target > nums[m] || target < nums[l] {
                l = m + 1;
            } else {
                r = m as i32 - 1;
            }
        } else {
            if target < nums[m] || target > nums[r as usize] {
                r = m as i32 - 1;
            } else {
                l = m + 1;
            }
        }
    }

    -1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = search(vec![4, 5, 6, 7, 0, 1, 2], 0);
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let res = search(vec![4, 5, 6, 7, 0, 1, 2], 3);
        assert_eq!(res, -1);
    }

    #[test]
    fn test3() {
        let res = search(vec![1], 0);
        assert_eq!(res, -1);
    }

    #[test]
    fn test4() {
        let res = search(vec![5, 1, 2], 5);
        assert_eq!(res, 0);
    }
}
