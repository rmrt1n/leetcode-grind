use std::collections::HashMap;

pub fn nc_is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();

    for i in 0..s.chars().count() {
        let char_s = s.chars().nth(i).unwrap();
        let char_t = t.chars().nth(i).unwrap();

        match map_s.get(&char_s) {
            Some(count) => map_s.insert(char_s, count + 1),
            None => map_s.insert(char_s, 0),
        };

        match map_t.get(&char_t) {
            Some(count) => map_t.insert(char_t, count + 1),
            None => map_t.insert(char_t, 0),
        };
    }

    map_s == map_t
}

pub fn is_anagram(s: String, t: String) -> bool {
    let mut map = HashMap::new();
    s.chars().for_each(|c| *map.entry(c).or_insert(0) += 1);
    t.chars().for_each(|c| *map.entry(c).or_insert(0) -= 1);
    map.into_values().all(|i| i == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nc_test1() {
        let res = nc_is_anagram(String::from("anagram"), String::from("nagaram"));
        assert_eq!(res, true);
    }

    #[test]
    fn nc_test2() {
        let res = nc_is_anagram(String::from("rat"), String::from("car"));
        assert_eq!(res, false);
    }

    #[test]
    fn test1() {
        let res = is_anagram(String::from("anagram"), String::from("nagaram"));
        assert_eq!(res, true);
    }

    #[test]
    fn test2() {
        let res = is_anagram(String::from("rat"), String::from("car"));
        assert_eq!(res, false);
    }
}
