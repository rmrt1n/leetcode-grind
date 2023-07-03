pub fn naive_daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![];

    for i in 0..temperatures.len() - 1 {
        let mut k = 0;
        for j in (i + 1)..temperatures.len() + 1 {
            if j == temperatures.len() {
                k = 0;
                break;
            }
            k += 1;
            if temperatures[j] - temperatures[i] > 0 {
                break;
            }
        }
        res.push(k);
    }

    res.push(0);
    res
}

pub fn nc_daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; temperatures.len()];
    let mut stack: Vec<(i32, i32)> = vec![];

    for (id, i) in temperatures.iter().enumerate() {
        while !stack.is_empty() && i > &stack.last().unwrap().1 {
            let (stack_id, _stack_i) = stack.pop().unwrap();
            res[stack_id as usize] = id as i32 - stack_id;
        }
        stack.push((id as i32, *i));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_test1() {
        let res = naive_daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        assert_eq!(res, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn naive_test2() {
        let res = naive_daily_temperatures(vec![30, 40, 50, 60]);
        assert_eq!(res, vec![1, 1, 1, 0]);
    }

    #[test]
    fn naive_test3() {
        let res = naive_daily_temperatures(vec![30, 60, 90]);
        assert_eq!(res, vec![1, 1, 0]);
    }

    #[test]
    fn nc_test1() {
        let res = nc_daily_temperatures(vec![73, 74, 75, 71, 69, 72, 76, 73]);
        assert_eq!(res, vec![1, 1, 4, 2, 1, 1, 0, 0]);
    }

    #[test]
    fn nc_test2() {
        let res = nc_daily_temperatures(vec![30, 40, 50, 60]);
        assert_eq!(res, vec![1, 1, 1, 0]);
    }

    #[test]
    fn nc_test3() {
        let res = nc_daily_temperatures(vec![30, 60, 90]);
        assert_eq!(res, vec![1, 1, 0]);
    }
}
