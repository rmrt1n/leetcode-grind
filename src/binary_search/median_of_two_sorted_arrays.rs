pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let total_len = nums1.len() + nums2.len();
    let mut m = (total_len - 1) / 2;
    let (mut l, mut r) = (0, 0);

    if nums1.is_empty() {
        if total_len % 2 == 0 {
            return (nums2[m] + nums2[m + 1]) as f64 / 2.0;
        }

        return nums2[m] as f64;
    }

    if nums2.is_empty() {
        if total_len % 2 == 0 {
            return (nums1[m] + nums1[m + 1]) as f64 / 2.0;
        }

        return nums1[m] as f64;
    }

    let (mut prev, mut cur) = (nums1[l], nums2[r]);

    while m > 0 && l < nums1.len() - 1 && r < nums2.len() - 1 {
        if nums1[l] <= nums2[r] {
            l += 1;
            (prev, cur) = (nums2[r], nums1[l]);
        } else {
            (prev, cur) = (nums1[l], nums2[r]);
            r += 1;
        }
        m -= 1;
    }

    while m > 0 && l < nums1.len() - 1 {
        if nums1[l] <= nums2[r] {
            (prev, cur) = (nums2[r], nums1[l + 1]);
        } else {
            (prev, cur) = (nums1[l], nums1[l + 1]);
        }
        l += 1;
        m -= 1;
    }

    while m > 0 && r < nums2.len() - 1 {
        if nums2[r] <= nums1[l] {
            (prev, cur) = (nums2[r + 1], nums1[l]);
        } else {
            (prev, cur) = (nums2[r], nums2[r + 1]);
        }
        r += 1;
        m -= 1;
    }

    if total_len % 2 == 0 {
        return (prev + cur) as f64 / 2.0;
    }

    prev as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = find_median_sorted_arrays(vec![1, 3], vec![2]);
        assert_eq!(res, 2.00000);
    }

    #[test]
    fn test2() {
        let res = find_median_sorted_arrays(vec![1, 2], vec![3, 4]);
        assert_eq!(res, 2.50000);
    }

    #[test]
    fn test3() {
        let res = find_median_sorted_arrays(vec![2, 3, 6], vec![1, 4, 5]);
        assert_eq!(res, 3.50000);
    }

    #[test]
    fn test4() {
        let res = find_median_sorted_arrays(vec![], vec![1]);
        assert_eq!(res, 1.00000);
    }

    #[test]
    fn test5() {
        let res = find_median_sorted_arrays(vec![3], vec![-2, -1]);
        assert_eq!(res, -1.00000);
    }
}
