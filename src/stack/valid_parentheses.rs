pub fn nc_is_valid(s: String) -> bool {
    let mut stack = vec![];

    for i in s.chars() {
        match i {
            '(' | '[' | '{' => stack.push(i),
            ')' => match stack.last() {
                Some(&c) => {
                    if c != '(' {
                        return false;
                    }
                    stack.pop();
                }
                None => stack.push(i),
            },
            ']' => match stack.last() {
                Some(&c) => {
                    if c != '[' {
                        return false;
                    }
                    stack.pop();
                }
                None => stack.push(i),
            },
            '}' => match stack.last() {
                Some(&c) => {
                    if c != '{' {
                        return false;
                    }
                    stack.pop();
                }
                None => stack.push(i),
            },
            _ => continue,
        }
    }

    stack.len() == 0
}

pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for i in s.chars() {
        match i {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' if Some(i) != stack.pop() => return false,
            _ => (),
        }
    }

    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_test1() {
        let res = nc_is_valid(String::from("()"));
        assert_eq!(res, true);
    }

    #[test]
    fn nc_test2() {
        let res = nc_is_valid(String::from("()[]{}"));
        assert_eq!(res, true);
    }

    #[test]
    fn nc_test3() {
        let res = nc_is_valid(String::from("(]"));
        assert_eq!(res, false);
    }

    #[test]
    fn nc_test4() {
        let res = nc_is_valid(String::from("{[]}"));
        assert_eq!(res, true);
    }

    #[test]
    fn test1() {
        let res = is_valid(String::from("()"));
        assert_eq!(res, true);
    }

    #[test]
    fn test2() {
        let res = is_valid(String::from("()[]{}"));
        assert_eq!(res, true);
    }

    #[test]
    fn test3() {
        let res = is_valid(String::from("(]"));
        assert_eq!(res, false);
    }

    #[test]
    fn test4() {
        let res = is_valid(String::from("{[]}"));
        assert_eq!(res, true);
    }
}
