use std::collections::HashMap;

pub fn nc_character_replacement(s: String, k: i32) -> i32 {
    let s = s.as_bytes();
    let mut map = HashMap::new();
    let (mut res, mut l, mut max_f) = (0, 0, 0);

    for (r, c) in s.iter().enumerate() {
        map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        max_f = max_f.max(*map.get(&c).unwrap());

        println!("map {:?} {c}", map);
        while (r - l + 1) - max_f > k as usize {
            map.entry(&s[l]).and_modify(|e| *e -= 1);
            l += 1;
        }
        res = res.max(r - l + 1);
    }

    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = nc_character_replacement(String::from("ABAB"), 2);
        assert_eq!(res, 4);
    }

    #[test]
    fn test2() {
        let res = nc_character_replacement(String::from("AABABBA"), 1);
        assert_eq!(res, 4);
    }
}
