pub fn encode(strs: Vec<String>) -> String {
    strs.into_iter()
        .map(|s| if s == ":" { String::from("::") } else { s })
        .collect::<Vec<String>>()
        .join(":;")
}

pub fn decode(str: String) -> Vec<String> {
    str.split(":;")
        .map(|s| {
            if s == "::" {
                String::from(":")
            } else {
                s.to_string()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let v = vec![
            String::from("lint"),
            String::from("code"),
            String::from("love"),
            String::from("you"),
        ];
        let res = encode(v.clone());
        assert_eq!(res, String::from("lint:;code:;love:;you"));

        let res = decode(res);
        assert_eq!(res, v);
    }

    #[test]
    fn test2() {
        let v = vec![
            String::from("we"),
            String::from("say"),
            String::from(":"),
            String::from("yes"),
        ];
        let res = encode(v.clone());
        assert_eq!(res, String::from("we:;say:;:::;yes"));

        let res = decode(res);
        assert_eq!(res, v);
    }
}
