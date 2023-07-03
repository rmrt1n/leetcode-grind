use std::{cmp::Ordering, collections::HashSet};

pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut map = HashSet::new();
    let mut set = HashSet::new();
    let mut sorted = nums.clone();
    sorted.sort_unstable();

    for (id, &i) in sorted[..sorted.len() - 2].iter().enumerate() {
        if map.contains(&i) {
            continue;
        }

        map.insert(i);
        let (mut l, mut r) = ((id + 1) as usize, sorted.len() - 1);

        while l < r {
            match (sorted[l] + sorted[r] + i).cmp(&0) {
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
                Ordering::Equal => {
                    let mut v = vec![i, sorted[l], sorted[r]];
                    v.sort_unstable();
                    if !set.contains(&v) {
                        set.insert(v);
                    }
                    l += 1;
                }
            }
        }
    }

    set.into_iter().collect()
}

pub fn nc_three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let mut sorted = nums.clone();
    sorted.sort_unstable();

    for (id, &i) in sorted[..sorted.len()].iter().enumerate() {
        if i > 0 || (id > 0 && i == sorted[id - 1]) {
            continue;
        }

        let (mut l, mut r) = ((id + 1) as usize, sorted.len() - 1);

        while l < r {
            match (sorted[l] + sorted[r] + i).cmp(&0) {
                Ordering::Less => l += 1,
                Ordering::Greater => r -= 1,
                Ordering::Equal => {
                    let v = vec![i, sorted[l], sorted[r]];
                    res.push(v);

                    l += 1;
                    r -= 1;
                    while sorted[l] == sorted[l - 1] && l < r {
                        l += 1;
                    }
                }
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut res = three_sum(vec![-1, 0, 1, 2, -1, -4]);
        res.sort();
        assert_eq!(res, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn test2() {
        let res = three_sum(vec![0, 1, 1]);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(res, empty);
    }

    #[test]
    fn test3() {
        let res = three_sum(vec![0, 0, 0]);
        assert_eq!(res, vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test4() {
        let res = three_sum(vec![1, 1, -2]);
        assert_eq!(res, vec![vec![-2, 1, 1]]);
    }

    #[test]
    fn nc_test1() {
        let mut res = nc_three_sum(vec![-1, 0, 1, 2, -1, -4]);
        res.sort();
        assert_eq!(res, vec![vec![-1, -1, 2], vec![-1, 0, 1]]);
    }

    #[test]
    fn nc_test2() {
        let res = nc_three_sum(vec![0, 1, 1]);
        println!("{:?}", res);
        let empty: Vec<Vec<i32>> = vec![];
        assert_eq!(res, empty);
    }

    #[test]
    fn nc_test3() {
        let res = nc_three_sum(vec![0, 0, 0]);
        assert_eq!(res, vec![vec![0, 0, 0]]);
    }

    #[test]
    fn nc_test4() {
        let res = nc_three_sum(vec![1, 1, -2]);
        assert_eq!(res, vec![vec![-2, 1, 1]]);
    }
}
