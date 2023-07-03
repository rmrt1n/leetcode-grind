use std::collections::HashMap;

// again for some reason idk this one is faster than nc
pub fn top_k_frequency_elements(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map: HashMap<i32, i32> = HashMap::new();

    nums.iter().for_each(|i| *map.entry(*i).or_insert(0) += 1);

    let mut x: Vec<(i32, i32)> = map.iter().map(|(a, b)| (*a, *b)).collect();

    x.sort_by(|a, b| b.1.cmp(&a.1));

    x[..k as usize].iter().map(|(a, _b)| *a).collect()
}

pub fn nc_top_k_frequency_elements(nums: Vec<i32>, k: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut freq: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];

    nums.iter().for_each(|i| *map.entry(*i).or_insert(0) += 1);

    map.iter().for_each(|(n, c)| freq[*c].push(*n));

    freq.into_iter().rev().flatten().take(k as usize).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = top_k_frequency_elements(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn test2() {
        let res = top_k_frequency_elements(vec![1], 1);
        assert_eq!(res, vec![1]);
    }

    #[test]
    fn nc_test1() {
        let res = nc_top_k_frequency_elements(vec![1, 1, 1, 2, 2, 3], 2);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn nc_test2() {
        let res = nc_top_k_frequency_elements(vec![1], 1);
        assert_eq!(res, vec![1]);
    }
}
