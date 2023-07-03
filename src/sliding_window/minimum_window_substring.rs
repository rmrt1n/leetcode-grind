use std::collections::HashMap;

pub fn min_window(s: String, t: String) -> String {
    if t.len() > s.len() {
        return String::new();
    }

    let mut map = HashMap::new();
    for i in t.as_bytes() {
        map.entry(*i).and_modify(|e| *e += 1).or_insert(1);
    }

    let dup = s.as_bytes();
    // have to reduce checking through map each time
    let (mut l, mut have) = (0, t.len());
    while l < s.len() && !map.contains_key(&dup[l]) {
        l += 1;
    }
    let mut res = (i32::MAX, 0, 0);

    for r in l..dup.len() {
        if map.contains_key(&dup[r]) {
            map.entry(dup[r]).and_modify(|e| *e -= 1);
            if *map.get(&dup[r]).unwrap() >= 0 {
                have -= 1;
            }
        }

        // println!("{}, {:?}, {have}", s[l..=r].to_string(), map);
        while have <= 0 {
            println!("debug {}, {:?}", s[l..=r].to_string(), map);
            let len = r as i32 - l as i32 + 1;
            if len < res.0 {
                res = (len, l, r)
            }
            map.entry(dup[l]).and_modify(|e| *e += 1);
            if *map.get(&dup[l]).unwrap() > 0 {
                have += 1;
            }
            l += 1;
            while l < s.len() && !map.contains_key(&dup[l]) {
                l += 1;
            }
        }
    }

    if res.0 == i32::MAX {
        // no matches
        return String::new();
    }

    s[res.1..=res.2].to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = min_window(String::from("ADOBECODEBANC"), String::from("ABC"));
        assert_eq!(res, String::from("BANC"));
    }

    #[test]
    fn test2() {
        let res = min_window(String::from("a"), String::from("a"));
        assert_eq!(res, String::from("a"));
    }

    #[test]
    fn test3() {
        let res = min_window(String::from("a"), String::from("aa"));
        assert_eq!(res, String::from(""));
    }

    #[test]
    fn test4() {
        let res = min_window(String::from("a"), String::from("b"));
        assert_eq!(res, String::from(""));
    }

    #[test]
    fn test5() {
        let res = min_window(String::from("ab"), String::from("a"));
        assert_eq!(res, String::from("a"));
    }

    #[test]
    fn test6() {
        let res = min_window(String::from("cabwefgewcwaefgcf"), String::from("cae"));
        assert_eq!(res, String::from("cwae"));
    }

    #[test]
    fn test7() {
        let res = min_window(String::from("bba"), String::from("ab"));
        assert_eq!(res, String::from("ba"));
    }
}
