use std::{cmp::Ordering, collections::HashMap};

#[allow(dead_code)]
struct TimeMap {
    values: HashMap<String, Vec<(i32, String)>>,
}

#[allow(dead_code)]
impl TimeMap {
    fn new() -> Self {
        TimeMap {
            values: HashMap::new(),
        }
    }

    fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.values.entry(key).or_insert(vec![]);
        entry.push((timestamp, value));
    }

    fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(s) = self.values.get(&key) {
            let (mut l, mut r): (usize, i32) = (0, s.len() as i32 - 1);
            let mut res = String::new();
            while l as i32 <= r {
                let m = l + (r as usize - l) / 2;
                match s[m].0.cmp(&timestamp) {
                    Ordering::Equal | Ordering::Less => {
                        res = s[m].1.clone();
                        l = m + 1;
                    }
                    Ordering::Greater => r = m as i32 - 1,
                }
            }
            return res;
        }
        String::from("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut tm = TimeMap::new();
        tm.set(String::from("foo"), String::from("bar"), 1);

        let res = tm.get(String::from("foo"), 1);
        assert_eq!(res, String::from("bar"));

        let res = tm.get(String::from("foo"), 3);
        assert_eq!(res, String::from("bar"));

        tm.set(String::from("foo"), String::from("bar2"), 4);

        let res = tm.get(String::from("foo"), 4);
        assert_eq!(res, String::from("bar2"));

        let res = tm.get(String::from("foo"), 5);
        assert_eq!(res, String::from("bar2"));
    }

    #[test]
    fn test2() {
        let mut tm = TimeMap::new();
        tm.set(String::from("love"), String::from("high"), 10);
        tm.set(String::from("love"), String::from("low"), 20);

        let res = tm.get(String::from("love"), 5);
        assert_eq!(res, String::from(""));

        let res = tm.get(String::from("love"), 10);
        assert_eq!(res, String::from("high"));

        let res = tm.get(String::from("love"), 15);
        assert_eq!(res, String::from("high"));

        let res = tm.get(String::from("love"), 20);
        assert_eq!(res, String::from("low"));

        let res = tm.get(String::from("love"), 25);
        assert_eq!(res, String::from("low"));
    }
}
