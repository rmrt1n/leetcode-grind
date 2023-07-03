use std::collections::HashSet;

pub fn nc_contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashSet::new();

    for i in nums.iter() {
        if map.contains(i) {
            return true;
        }
        map.insert(i);
    }

    false
}

pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map = HashSet::new();
    !nums.iter().all(|c| map.insert(c))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_test1() {
        let res = nc_contains_duplicate(vec![1, 2, 3, 1]);
        assert_eq!(res, true);
    }

    #[test]
    fn nc_test2() {
        let res = nc_contains_duplicate(vec![1, 2, 3, 4]);
        assert_eq!(res, false);
    }

    #[test]
    fn nc_test3() {
        let res = nc_contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert_eq!(res, true);
    }

    #[test]
    fn test1() {
        let res = contains_duplicate(vec![1, 2, 3, 1]);
        assert_eq!(res, true);
    }

    #[test]
    fn test2() {
        let res = contains_duplicate(vec![1, 2, 3, 4]);
        assert_eq!(res, false);
    }

    #[test]
    fn test3() {
        let res = contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]);
        assert_eq!(res, true);
    }
}
