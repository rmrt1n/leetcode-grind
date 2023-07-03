use std::collections::VecDeque;

pub fn naive_max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = vec![];

    for i in 0..(nums.len() - k as usize + 1usize) {
        let mut max = nums[i];
        for j in (i + 1)..(i + k as usize) {
            max = max.max(nums[j]);
        }
        res.push(max);
    }

    res
}

pub fn failed_max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = vec![];
    let (mut prev_max, mut prev_id) = (nums[0], 0usize);

    for i in 0..(k as usize) {
        if nums[i] > prev_max {
            prev_max = nums[i];
            prev_id = i;
        }
    }
    res.push(prev_max);

    let (mut l, mut r) = (1usize, k as usize);

    while r < nums.len() {
        if l > prev_id as usize {
            prev_max = nums[l];
            prev_id = l;
            for i in l..r {
                if nums[i] > prev_max {
                    prev_max = nums[i];
                    prev_id = i;
                }
            }
        }

        if nums[r] > prev_max {
            prev_max = nums[r];
            prev_id = r;
        }
        res.push(prev_max);

        l += 1;
        r += 1;
    }

    res
}

pub fn nc_max_sliding_window(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut res = vec![];
    let mut q: VecDeque<usize> = VecDeque::new();

    let (mut l, mut r) = (0, 0);

    while r < nums.len() {
        while !q.is_empty() && nums[r] > nums[*q.back().unwrap()] {
            q.pop_back();
        }

        q.push_back(r);

        if l > *q.front().unwrap() {
            q.pop_front();
        }

        if r + 1 >= k as usize {
            res.push(nums[*q.front().unwrap()]);
            l += 1;
        }

        r += 1;
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_test1() {
        let res = naive_max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(res, vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn naive_test2() {
        let res = naive_max_sliding_window(vec![1], 1);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn naive_test3() {
        let res = naive_max_sliding_window(vec![1, -1], 1);
        assert_eq!(res, vec![1, -1]);
    }

    #[test]
    fn failed_test1() {
        let res = failed_max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(res, vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn failed_test2() {
        let res = failed_max_sliding_window(vec![1], 1);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn failed_test3() {
        let res = failed_max_sliding_window(vec![1, -1], 1);
        assert_eq!(res, vec![1, -1]);
    }

    #[test]
    fn test1() {
        let res = nc_max_sliding_window(vec![1, 3, -1, -3, 5, 3, 6, 7], 3);
        assert_eq!(res, vec![3, 3, 5, 5, 6, 7]);
    }

    #[test]
    fn test2() {
        let res = nc_max_sliding_window(vec![1], 1);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn test3() {
        let res = nc_max_sliding_window(vec![1, -1], 1);
        assert_eq!(res, vec![1, -1]);
    }
}
