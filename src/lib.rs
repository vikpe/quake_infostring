use std::collections::HashMap;

mod util;

pub const DELIMITER: char = '\\';

pub fn to_hashmap(serverinfo: &str) -> HashMap<String, String> {
    let serverinfo_ = util::clean(serverinfo);
    let mut iter = serverinfo_
        .trim_matches(DELIMITER)
        .split(DELIMITER)
        .map(|v| v.to_string());
    let mut result = HashMap::new();

    while let Some(key) = iter.next() {
        if let Some(value) = iter.next() {
            result.insert(key, value);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_to_hashmap() {
        // unbalanced string
        {
            let result = to_hashmap(r#"\maxfps\77"#);
            assert_eq!(result.get("maxfps"), Some(&"77".to_string()));
            assert_eq!(result.get("matchtag"), None);
        }

        // valid string
        {
            let result = to_hashmap(r#"\maxfps\77\matchtag\kombat\epoch\123456"#);
            assert_eq!(result.get("maxfps"), Some(&"77".to_string()));
            assert_eq!(result.get("matchtag"), Some(&"kombat".to_string()));
            assert_eq!(result.get("epoch"), Some(&"123456".to_string()));

            assert_eq!(result.get("MISSING_KEY"), None);
        }
    }
}
