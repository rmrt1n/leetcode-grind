use std::cmp::Ordering;

pub fn trap(height: Vec<i32>) -> i32 {
    let mut res = 0;
    let (mut i, mut j) = (0, 1);
    let mut stack = vec![];

    while j < height.len() {
        if height[j] >= height[i] {
            if j - i == 1 {
                j += 1;
                i += 1;
                continue;
            }

            let acc = stack.iter().fold(0, |acc, i| acc + i);
            res += height[i] * (j as i32 - i as i32 - 1) - acc;
            i = j;
            j += 1;
            stack.clear();
            continue;
        }
        stack.push(height[j]);
        j += 1;
    }

    if i == height.len() - 1 {
        return res;
    }

    j -= 1;
    let mut k = j - 1;
    stack.clear();

    while k >= i {
        println!("fuck {j} {k} {i} {}", k == i);
        if height[k] >= height[j] {
            if j - k == 1 {
                if k == i {
                    break;
                };
                k -= 1;
                j -= 1;
                continue;
            }

            let acc = stack.iter().fold(0, |acc, i| acc + i);
            println!("{acc} {res}");
            res += height[j] * (j as i32 - k as i32 - 1) - acc;
            if k == i {
                break;
            };
            j = k;
            k -= 1;
            stack.clear();
            continue;
        }
        stack.push(height[k]);
        k -= 1;
    }

    res
}

pub fn nc_trap(height: Vec<i32>) -> i32 {
    if height.is_empty() {
        return 0;
    }

    let (mut max_l, mut max_r) = (height.first().unwrap(), height.last().unwrap());
    let (mut l, mut r) = (0, height.len() - 1);
    let mut res = 0;

    while l < r {
        match max_l.cmp(&max_r) {
            Ordering::Less | Ordering::Equal => {
                l += 1;
                max_l = max_l.max(&height[l]);
                res += max_l - height[l];
            }
            Ordering::Greater => {
                r -= 1;
                max_r = max_r.max(&height[r]);
                res += max_r - height[r];
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
        let res = trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(res, 6);
    }

    #[test]
    fn test2() {
        let res = trap(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(res, 9);
    }

    #[test]
    fn test3() {
        let res = trap(vec![4, 2, 3]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test4() {
        let res = trap(vec![
            1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984,
            983, 982, 981, 980, 979, 978, 977, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
        ]);
        assert_eq!(res, 0);
    }

    #[test]
    fn nc_test1() {
        let res = nc_trap(vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1]);
        assert_eq!(res, 6);
    }

    #[test]
    fn nc_test2() {
        let res = nc_trap(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(res, 9);
    }

    #[test]
    fn nc_test3() {
        let res = nc_trap(vec![4, 2, 3]);
        assert_eq!(res, 1);
    }

    #[test]
    fn nc_test4() {
        let res = nc_trap(vec![
            1000, 999, 998, 997, 996, 995, 994, 993, 992, 991, 990, 989, 988, 987, 986, 985, 984,
            983, 982, 981, 980, 979, 978, 977, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
            966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966, 966,
        ]);
        assert_eq!(res, 0);
    }
}
