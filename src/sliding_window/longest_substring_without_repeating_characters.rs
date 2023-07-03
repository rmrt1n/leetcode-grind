use std::collections::HashSet;

pub fn naive_length_of_longest_substring(s: String) -> i32 {
    if s.len() == 0 {
        return 0;
    }

    let mut r = 1;
    let mut max = 0;
    let mut str = s[..=0].to_string();
    let chars: Vec<char> = s.chars().collect();

    while r < chars.len() {
        if !str.contains(chars[r]) {
            str.push(chars[r]);
            r += 1;
            continue;
        }
        max = max.max(str.len());
        str = str[1..].to_string();
    }

    max = max.max(str.len());
    max as i32
}

pub fn nc_length_of_longest_substring(s: String) -> i32 {
    let mut set = HashSet::new();
    let mut l = 0;
    let mut res = 0;
    let chars: Vec<char> = s.chars().collect();

    for (r, c) in chars.iter().enumerate() {
        while set.contains(&c) {
            set.remove(&chars[l]);
            l += 1;
        }
        set.insert(c);
        res = res.max(r - l + 1);
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = naive_length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(res, 3);
    }

    #[test]
    fn test2() {
        let res = naive_length_of_longest_substring(String::from("bbbbb"));
        assert_eq!(res, 1);
    }

    #[test]
    fn test3() {
        let res = naive_length_of_longest_substring(String::from("pwwkew"));
        assert_eq!(res, 3);
    }

    #[test]
    fn test4() {
        let res = naive_length_of_longest_substring(String::from("dvdf"));
        assert_eq!(res, 3);
    }

    #[test]
    fn test5() {
        let res = naive_length_of_longest_substring(String::from(""));
        assert_eq!(res, 0);
    }

    #[test]
    fn nc_test1() {
        let res = nc_length_of_longest_substring(String::from("abcabcbb"));
        assert_eq!(res, 3);
    }

    #[test]
    fn nc_test2() {
        let res = nc_length_of_longest_substring(String::from("bbbbb"));
        assert_eq!(res, 1);
    }

    #[test]
    fn nc_test3() {
        let res = nc_length_of_longest_substring(String::from("pwwkew"));
        assert_eq!(res, 3);
    }

    #[test]
    fn nc_test4() {
        let res = nc_length_of_longest_substring(String::from("dvdf"));
        assert_eq!(res, 3);
    }

    #[test]
    fn nc_test5() {
        let res = nc_length_of_longest_substring(String::from(""));
        assert_eq!(res, 0);
    }
}
