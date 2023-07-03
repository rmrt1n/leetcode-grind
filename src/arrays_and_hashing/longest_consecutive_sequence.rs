use std::collections::HashSet;

pub fn nc_longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = HashSet::from_iter(nums.clone());
    let mut res = 0;

    for i in nums {
        match set.get(&(i - 1)) {
            Some(_) => continue,
            None => {
                let mut j = 0;
                while let Some(_) = set.get(&(i + j)) {
                    j += 1;
                }
                if j > res {
                    res = j;
                }
            }
        }
    }

    res
}

// the rust :3 way
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = HashSet::from_iter(nums.clone());

    nums.into_iter()
        .filter(|&x| !set.contains(&(x - 1)))
        .map(|x| (x..).take_while(|x| set.contains(x)).count())
        .max()
        .map_or(0, |x| x as i32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_test1() {
        let res = nc_longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert_eq!(res, 4);
    }

    #[test]
    fn nc_test2() {
        let res = nc_longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        assert_eq!(res, 9);
    }

    #[test]
    fn nc_test3() {
        let res = nc_longest_consecutive(vec![]);
        assert_eq!(res, 0);
    }

    #[test]
    fn test1() {
        let res = longest_consecutive(vec![100, 4, 200, 1, 3, 2]);
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let res = longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]);
        assert_eq!(res, 9);
    }

    #[test]
    fn test3() {
        let res = longest_consecutive(vec![]);
        assert_eq!(res, 0);
    }
}
