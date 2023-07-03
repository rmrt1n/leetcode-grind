pub fn naive_largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let coordinates: Vec<(usize, usize)> = (0..heights.len())
        .map(|i| (i, heights[i] as usize))
        .collect();
    let mut max = 0;

    for (x1, max_h) in &coordinates {
        for y in 1..=*max_h {
            for x2 in (x1 + 1)..=heights.len() {
                if y > coordinates[x2 - 1].1 {
                    break;
                }
                let area = (x2 - x1) * y;
                if area > max {
                    println!("({x1}, 0) ({x2}, {y}), {area}");
                    max = area;
                }
            }
        }
    }

    max as i32
}

pub fn nc_largest_rectangle_area(heights: Vec<i32>) -> i32 {
    let mut max = 0;
    let mut stack: Vec<(usize, i32)> = vec![];

    for (id, i) in heights.iter().enumerate() {
        let mut last_id = id;
        while !stack.is_empty() && i < &stack.last().unwrap().1 {
            let (top_id, top_i) = stack.pop().unwrap();
            last_id = top_id;
            max = max.max(top_i * (id - top_id) as i32);
        }
        stack.push((last_id, *i))
    }

    for (id, i) in stack.iter() {
        max = max.max(*i * (heights.len() - id) as i32);
    }

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn naive_test1() {
        let res = naive_largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]);
        assert_eq!(res, 10);
    }

    #[test]
    fn naive_test2() {
        let res = naive_largest_rectangle_area(vec![2, 4]);
        assert_eq!(res, 4);
    }

    #[test]
    fn naive_test3() {
        let res = naive_largest_rectangle_area(vec![0]);
        assert_eq!(res, 0);
    }

    #[test]
    fn naive_test4() {
        let res = naive_largest_rectangle_area(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(res, 6);
    }

    #[test]
    fn nc_test1() {
        let res = nc_largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]);
        assert_eq!(res, 10);
    }

    #[test]
    fn nc_test2() {
        let res = nc_largest_rectangle_area(vec![2, 4]);
        assert_eq!(res, 4);
    }

    #[test]
    fn nc_test3() {
        let res = nc_largest_rectangle_area(vec![0]);
        assert_eq!(res, 0);
    }

    #[test]
    fn nc_test4() {
        let res = nc_largest_rectangle_area(vec![4, 2, 0, 3, 2, 5]);
        assert_eq!(res, 6);
    }
}
