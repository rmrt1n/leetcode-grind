use std::collections::HashMap;

// this one is actually faster than nc, prob due to some optimizations by rust
// or my nc implementation is just shit
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for i in strs {
        let mut sorted: Vec<char> = i.chars().collect();
        sorted.sort_unstable();

        let entry = map.entry(sorted).or_insert(Vec::<String>::new());
        entry.push(i);
    }

    map.into_values().collect()
}

pub fn nc_group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map = HashMap::new();

    for s in strs {
        let mut count = vec![0; 26];

        s.bytes().for_each(|b| count[(b - b'a') as usize] += 1);

        map.entry(count).or_insert(vec![]).push(s);
    }

    map.into_values().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let res = group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ])
        .sort();
        assert_eq!(
            res,
            vec![
                vec![String::from("bat")],
                vec![String::from("tan"), String::from("nat")],
                vec![
                    String::from("ate"),
                    String::from("eat"),
                    String::from("tea")
                ]
            ]
            .sort()
        );
    }

    #[test]
    fn test2() {
        let res = group_anagrams(vec![String::from("")]);
        assert_eq!(res, vec![vec![String::from("")]]);
    }

    #[test]
    fn test3() {
        let res = group_anagrams(vec![String::from("a")]);
        assert_eq!(res, vec![vec![String::from("a")]]);
    }

    #[test]
    fn nc_test1() {
        let res = nc_group_anagrams(vec![
            String::from("eat"),
            String::from("tea"),
            String::from("tan"),
            String::from("ate"),
            String::from("nat"),
            String::from("bat"),
        ])
        .sort();
        assert_eq!(
            res,
            vec![
                vec![String::from("bat")],
                vec![String::from("tan"), String::from("nat")],
                vec![
                    String::from("ate"),
                    String::from("eat"),
                    String::from("tea")
                ]
            ]
            .sort()
        );
    }

    #[test]
    fn nc_test2() {
        let res = nc_group_anagrams(vec![String::from("")]);
        assert_eq!(res, vec![vec![String::from("")]]);
    }

    #[test]
    fn nc_test3() {
        let res = nc_group_anagrams(vec![String::from("a")]);
        assert_eq!(res, vec![vec![String::from("a")]]);
    }
}
