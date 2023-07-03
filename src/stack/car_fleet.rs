use std::collections::HashMap;

// didn't read that car's can't pass each other as there's only 1 lane
pub fn failed_car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    fn helper(stack: Vec<(i32, i32)>, target: i32, acc: i32) -> i32 {
        if stack.is_empty() {
            return acc;
        }

        if stack.len() == 1 {
            return acc + 1;
        }

        let mut map = HashMap::new();

        for (pos, speed) in stack {
            let new_pos = pos + speed;
            let entry = map.entry(new_pos).or_insert((new_pos, speed));

            if speed < entry.1 {
                entry.1 = speed;
            }
        }

        let new_acc = map
            .clone()
            .into_iter()
            .fold(acc, |acc, (pos, _)| acc + if pos >= target { 1 } else { 0 });

        let new_stack: Vec<(i32, i32)> = map
            .into_values()
            .filter(|(pos, _)| *pos != target && *pos < target)
            .collect();

        helper(new_stack, target, new_acc)
    }

    helper(
        position.into_iter().zip(speed.into_iter()).collect(),
        target,
        0,
    )
}

pub fn nc_car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut pairs: Vec<(f64, f64)> = position
        .iter()
        .map(|i| *i as f64)
        .zip(speed.iter().map(|i| *i as f64))
        .collect();
    pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    let mut stack = vec![];

    for (pos, speed) in pairs {
        stack.push((target as f64 - pos) / speed); // time needed to reach target
        if stack.len() >= 2 && stack.last() <= stack.get(stack.len() - 2) {
            stack.pop();
        }
    }

    stack.len() as i32
}

pub fn car_fleet(target: i32, position: Vec<i32>, speed: Vec<i32>) -> i32 {
    let mut pairs: Vec<(i32, i32)> = position.into_iter().zip(speed.into_iter()).collect();
    pairs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    pairs
        .into_iter()
        .map(|(pos, speed)| (target as f64 - pos as f64) / speed as f64)
        .fold((0, 0.0), |(acc, longest), time| {
            if time > longest {
                (acc + 1, time)
            } else {
                (acc, longest)
            }
        })
        .0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn failed_test1() {
        let res = failed_car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]);
        assert_eq!(res, 3);
    }

    #[test]
    fn failed_test2() {
        let res = failed_car_fleet(10, vec![3], vec![3]);
        assert_eq!(res, 1);
    }

    #[test]
    fn failed_test3() {
        let res = failed_car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]);
        assert_eq!(res, 1);
    }

    #[test]
    fn failed_test4() {
        let res = failed_car_fleet(10, vec![4, 6], vec![3, 2]);
        assert_eq!(res, 1);
    }

    #[test]
    fn failed_test5() {
        let res = failed_car_fleet(10, vec![8, 3, 7, 4, 6, 5], vec![4, 4, 4, 4, 4, 4]);
        assert_eq!(res, 6);
    }

    #[test]
    fn nc_test1() {
        let res = nc_car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]);
        assert_eq!(res, 3);
    }

    #[test]
    fn nc_test2() {
        let res = nc_car_fleet(10, vec![3], vec![3]);
        assert_eq!(res, 1);
    }

    #[test]
    fn nc_test3() {
        let res = nc_car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]);
        assert_eq!(res, 1);
    }

    #[test]
    fn nc_test4() {
        let res = nc_car_fleet(10, vec![4, 6], vec![3, 2]);
        assert_eq!(res, 1);
    }

    #[test]
    fn nc_test5() {
        let res = nc_car_fleet(10, vec![8, 3, 7, 4, 6, 5], vec![4, 4, 4, 4, 4, 4]);
        assert_eq!(res, 6);
    }

    #[test]
    fn test1() {
        let res = car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]);
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let res = car_fleet(10, vec![3], vec![3]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test3() {
        let res = car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test4() {
        let res = car_fleet(10, vec![4, 6], vec![3, 2]);
        assert_eq!(res, 1);
    }

    #[test]
    fn test5() {
        let res = car_fleet(10, vec![8, 3, 7, 4, 6, 5], vec![4, 4, 4, 4, 4, 4]);
        assert_eq!(res, 6);
    }
}
