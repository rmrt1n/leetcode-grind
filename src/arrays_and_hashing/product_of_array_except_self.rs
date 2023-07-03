pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![1; nums.len()];
    let (mut prefix, mut postfix) = (1, 1);

    for i in 0..nums.len() {
        res[i] = prefix;
        prefix *= nums[i];
    }

    for i in (0..nums.len()).rev() {
        res[i] *= postfix;
        postfix *= nums[i];
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = product_except_self(vec![1, 2, 3, 4]);
        assert_eq!(res, vec![24, 12, 8, 6]);
    }

    #[test]
    fn test2() {
        let res = product_except_self(vec![-1, 1, 0, -3, 3]);
        assert_eq!(res, vec![0, 0, 9, 0, 0]);
    }
}
