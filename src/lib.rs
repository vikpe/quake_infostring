//! # quake_infostring
//! Parse QuakeWorld info strings
use std::collections::HashMap;

mod util;

const DELIMITER: char = '\\';

/// Convert info string to HashMap
///
/// # Examples
/// ```
/// let info_str = r#"\maxfps\77\matchtag\kombat"#;
/// let info = quake_infostring::to_hashmap(&info_str);
///
/// assert_eq!(info.get("maxfps"), Some(&"77".to_string()));
/// assert_eq!(info.get("matchtag"), Some(&"kombat".to_string()));
/// assert_eq!(info.get("MISSING_KEY"), None);
/// ```
pub fn to_hashmap(info: &str) -> HashMap<String, String> {
    let info_ = util::clean(info);
    let mut iter = info_
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
    }
}
