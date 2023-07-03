use std::collections::HashMap;

pub fn naive_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (id_i, i) in nums.iter().enumerate() {
        for j in (id_i + 1)..nums.len() {
            if i + nums[j] == target {
                return vec![id_i as i32, j as i32];
            }
        }
    }

    vec![]
}

pub fn nc_two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (id, i) in nums.iter().enumerate() {
        let diff = target - i;
        if map.contains_key(&diff) {
            return vec![*map.get(&diff).unwrap(), id as i32];
        }
        map.insert(i, id as i32);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_test1() {
        let res = naive_two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(res, vec![0, 1]);
    }

    #[test]
    fn naive_test2() {
        let res = naive_two_sum(vec![3, 3], 6);
        assert_eq!(res, vec![0, 1]);
    }

    #[test]
    fn naive_test3() {
        let res = naive_two_sum(vec![3, 2, 4], 6);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn test1() {
        let res = nc_two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(res, vec![0, 1]);
    }

    #[test]
    fn test2() {
        let res = nc_two_sum(vec![3, 3], 6);
        assert_eq!(res, vec![0, 1]);
    }

    #[test]
    fn test3() {
        let res = nc_two_sum(vec![3, 2, 4], 6);
        assert_eq!(res, vec![1, 2]);
    }
}
