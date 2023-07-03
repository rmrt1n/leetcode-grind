use std::cmp::Ordering;

pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut l, mut r) = (0, matrix.len() - 1);

    while l <= r {
        let m = l + (r - l) / 2;
        match matrix[m as usize][0].cmp(&target) {
            Ordering::Equal => return true,
            ord => {
                let (mut left, mut right) = (1, matrix[m as usize].len() - 1);
                while left <= right {
                    let mid = left + (right - left) / 2;
                    match matrix[m][mid].cmp(&target) {
                        Ordering::Equal => return true,
                        Ordering::Less => left = mid + 1,
                        Ordering::Greater => right = mid - 1,
                    }
                }
                match ord {
                    Ordering::Less => l = m + 1,
                    Ordering::Greater => r = m - 1,
                    Ordering::Equal => unreachable!(),
                }
            }
        }
    }

    false
}

// cibai this fuckin shit is much longer but performs worse in nc?
pub fn nc_search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
    let (mut t, mut b): (usize, i32) = (0, matrix.len() as i32 - 1);

    while t as i32 <= b {
        let m = t + (b as usize - t) / 2;
        if matrix[m][matrix[0].len() - 1] < target {
            t = m + 1;
        } else if matrix[m][0] > target {
            b = m as i32 - 1;
        } else {
            break;
        }
    }

    if !(t as i32 <= b) {
        return false;
    }

    let mid = t + (b as usize - t) / 2;
    let (mut l, mut r) = (0, matrix[mid].len() - 1);

    while l <= r {
        let m = l + (r - l) / 2;
        match matrix[mid][m].cmp(&target) {
            Ordering::Equal => return true,
            Ordering::Less => l = m + 1,
            Ordering::Greater => r = m - 1,
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3,
        );
        assert_eq!(res, true);
    }

    #[test]
    fn test2() {
        let res = search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13,
        );
        assert_eq!(res, false);
    }

    #[test]
    fn nc_test1() {
        let res = nc_search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            3,
        );
        assert_eq!(res, true);
    }

    #[test]
    fn nc_test2() {
        let res = nc_search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 60]],
            13,
        );
        assert_eq!(res, false);
    }

    #[test]
    fn nc_test3() {
        let res = nc_search_matrix(vec![vec![1, 1]], 2);
        assert_eq!(res, false);
    }
}
