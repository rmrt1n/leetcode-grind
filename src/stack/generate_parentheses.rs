pub fn nc_generate_parentheses(n: i32) -> Vec<String> {
    let mut res: Vec<String> = vec![];

    fn backtrack(res: &mut Vec<String>, s: String, open: i32, close: i32) {
        if open == 0 && close == 0 {
            res.push(s);
            return;
        }
        if open == close {
            backtrack(res, s.clone() + "(", open - 1, close);
        } else {
            if open > 0 {
                backtrack(res, s.clone() + "(", open - 1, close);
            }
            if close > 0 {
                backtrack(res, s.clone() + ")", open, close - 1);
            }
        }
    }

    backtrack(&mut res, String::from(""), n, n);
    res
}

// this one looks so slick but the above one is more performant sadly :(
// pattern matching must be costly
pub fn generate_parentheses(n: i32) -> Vec<String> {
    fn gen_paren(open: i32, close: i32, s: String, acc: Vec<String>) -> Vec<String> {
        println!("shit {open} {close} {s} {:?}", acc);
        match (open, close) {
            (0, 0) => vec![s],
            _ if open > 0 && close > 0 => vec![
                gen_paren(open - 1, close + 1, s.clone() + "(", acc.clone()),
                gen_paren(open, close - 1, s + ")", acc),
            ]
            .concat(),
            _ if open > 0 => gen_paren(open - 1, close + 1, s + "(", acc),
            _ if close > 0 => gen_paren(open, close - 1, s + ")", acc),
            _ => unreachable!(),
        }
    }
    gen_paren(n, 0, String::new(), vec![])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_test1() {
        let res = nc_generate_parentheses(1);
        assert_eq!(res, vec![String::from("()")]);
    }

    #[test]
    fn nc_test2() {
        let res = nc_generate_parentheses(2);
        assert_eq!(res, vec![String::from("(())"), String::from("()()")]);
    }

    #[test]
    fn nc_test3() {
        let res = nc_generate_parentheses(3);
        assert_eq!(
            res,
            vec![
                String::from("((()))"),
                String::from("(()())"),
                String::from("(())()"),
                String::from("()(())"),
                String::from("()()()"),
            ]
        );
    }

    #[test]
    fn test1() {
        let res = generate_parentheses(1);
        assert_eq!(res, vec![String::from("()")]);
    }

    #[test]
    fn test2() {
        let res = generate_parentheses(2);
        assert_eq!(res, vec![String::from("(())"), String::from("()()")]);
    }

    #[test]
    fn test3() {
        let res = generate_parentheses(3);
        assert_eq!(
            res,
            vec![
                String::from("((()))"),
                String::from("(()())"),
                String::from("(())()"),
                String::from("()(())"),
                String::from("()()()"),
            ]
        );
    }
}
