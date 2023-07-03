use std::collections::HashMap;

pub fn check_inclusion(s1: String, s2: String) -> bool {
    if s2.len() < s1.len() {
        return false;
    }

    let mut map = HashMap::new();

    for i in s1.as_bytes() {
        map.entry(*i).and_modify(|e| *e += 1).or_insert(1);
    }

    let s2 = s2.as_bytes();
    for i in 0..(s2.len() - s1.len() + 1) {
        if !map.contains_key(&s2[i]) {
            continue;
        }

        let mut map2 = HashMap::new();
        for j in i..(i + s1.len()) {
            map2.entry(s2[j]).and_modify(|e| *e += 1).or_insert(1);
        }
        if map2 == map {
            return true;
        }
    }

    false
}

pub fn nc_check_inclusion(s1: String, s2: String) -> bool {
    if s2.len() < s1.len() {
        return false;
    }

    let (s1, s2) = (s1.as_bytes(), s2.as_bytes());
    let (mut map1, mut map2) = ([0; 26], [0; 26]);
    for i in 0..s1.len() {
        map1[(s1[i] - b'a') as usize] += 1;
        map2[(s2[i] - b'a') as usize] += 1;
    }

    let mut matches = (0..26).fold(0, |acc, i| acc + if map1[i] == map2[i] { 1 } else { 0 });
    let mut l = 0;

    for r in s1.len()..s2.len() {
        if matches == 26 {
            return true;
        }

        let id = (s2[r] - b'a') as usize;
        map2[id] += 1;
        if map1[id] == map2[id] {
            matches += 1;
        } else if map1[id] + 1 == map2[id] {
            matches -= 1;
        }

        let id = (s2[l] - b'a') as usize;
        map2[id] -= 1;
        if map1[id] == map2[id] {
            matches += 1;
        } else if map1[id] - 1 == map2[id] {
            matches -= 1;
        }

        l += 1
    }

    matches == 26
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = check_inclusion(String::from("ab"), String::from("eidbaooo"));
        assert_eq!(res, true);
    }

    #[test]
    fn test2() {
        let res = check_inclusion(String::from("ab"), String::from("eidboaoo"));
        assert_eq!(res, false);
    }

    #[test]
    fn test3() {
        let res = check_inclusion(String::from("horse"), String::from("ros"));
        assert_eq!(res, false);
    }

    #[test]
    fn nc_test1() {
        let res = nc_check_inclusion(String::from("ab"), String::from("eidbaooo"));
        assert_eq!(res, true);
    }

    #[test]
    fn nc_test2() {
        let res = nc_check_inclusion(String::from("ab"), String::from("eidboaoo"));
        assert_eq!(res, false);
    }

    #[test]
    fn nc_test3() {
        let res = nc_check_inclusion(String::from("horse"), String::from("ros"));
        assert_eq!(res, false);
    }

    #[test]
    fn nc_test4() {
        let res = nc_check_inclusion(String::from("abc"), String::from("bbbca"));
        assert_eq!(res, true);
    }

    #[test]
    fn nc_test5() {
        let res = nc_check_inclusion(String::from("rokx"), String::from("otrxvfszxroxrzdsltg"));
        assert_eq!(res, false);
    }
}
