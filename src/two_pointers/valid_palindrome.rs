pub fn is_palindrome(s: String) -> bool {
    let s: Vec<char> = s
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect();

    for i in 0..(s.len() / 2) {
        if s[i] != s[s.len() - i - 1] {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = is_palindrome(String::from("A man, a plan, a canal: Panama"));
        assert_eq!(res, true);
    }

    #[test]
    fn test2() {
        let res = is_palindrome(String::from("race a car"));
        assert_eq!(res, false);
    }

    #[test]
    fn test3() {
        let res = is_palindrome(String::from(" "));
        assert_eq!(res, true);
    }
}
