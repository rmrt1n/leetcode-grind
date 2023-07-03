use std::cmp::Ordering::{Equal, Greater, Less};

pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;

    while numbers[i] + numbers[j] != target {
        while numbers[i] + numbers[j] > target && j > i {
            j -= 1;
        }

        while numbers[i] + numbers[j] < target && i < j - 1 {
            i += 1;
        }
    }

    vec![(i + 1) as i32, (j + 1) as i32]
}

pub fn nc_two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut i = 0;
    let mut j = numbers.len() - 1;

    while i < j {
        match (numbers[i] + numbers[j]).cmp(&target) {
            Less => i += 1,
            Greater => j -= 1,
            Equal => return vec![(i + 1) as i32, (j + 1) as i32],
        }
    }

    unreachable!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = two_sum(vec![2, 7, 11, 15], 9);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn test2() {
        let res = two_sum(vec![2, 3, 4], 6);
        assert_eq!(res, vec![1, 3]);
    }

    #[test]
    fn test3() {
        let res = two_sum(vec![-1, 0], -1);
        assert_eq!(res, vec![1, 2]);
    }

    #[test]
    fn test4() {
        let res = two_sum(vec![3, 24, 50, 79, 88, 150, 345], 200);
        assert_eq!(res, vec![3, 6]);
    }
}
