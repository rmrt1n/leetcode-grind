pub fn eval_rpn(tokens: Vec<String>) -> i32 {
    let mut stack = vec![];

    for i in tokens {
        match i.as_str() {
            "+" => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l + r);
            }
            "-" => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l - r);
            }
            "*" => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                stack.push(l * r);
            }
            "/" => {
                let r = stack.pop().unwrap();
                let l = stack.pop().unwrap();
                println!("{l} / {r} = {}", l / r);
                stack.push(l / r);
            }
            s => {
                if let Ok(x) = s.parse::<i32>() {
                    stack.push(x);
                }
            }
        }
    }

    *stack.last().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = eval_rpn(vec![
            String::from("2"),
            String::from("1"),
            String::from("+"),
            String::from("3"),
            String::from("*"),
        ]);
        assert_eq!(res, 9);
    }

    #[test]
    fn test2() {
        let res = eval_rpn(vec![
            String::from("4"),
            String::from("13"),
            String::from("5"),
            String::from("/"),
            String::from("+"),
        ]);
        assert_eq!(res, 6);
    }

    #[test]
    fn test3() {
        let res = eval_rpn(vec![
            String::from("10"),
            String::from("6"),
            String::from("9"),
            String::from("3"),
            String::from("+"),
            String::from("-11"),
            String::from("*"),
            String::from("/"),
            String::from("*"),
            String::from("17"),
            String::from("+"),
            String::from("5"),
            String::from("+"),
        ]);
        assert_eq!(res, 22);
    }
}
