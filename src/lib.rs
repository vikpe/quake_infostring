#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

//! # quake_infostring
//! Parse QuakeWorld info strings
use std::collections::HashMap;

const KV_DELIMITER: char = '\\';

/// Parse a delimited key-value string into a `HashMap`.
///
/// The input string is expected to contain pairs of keys and values separated by a delimiter (default: `\`).
/// Leading and trailing delimiters are ignored. The function sanitizes the input by removing quotes,
/// unescaping double delimiters, and trimming whitespace.
///
/// If the input contains an odd number of segments, the last key without a corresponding value is ignored.
///
/// # Examples
///
/// ```
/// let info = r#"\maxfps\77\matchtag\kombat"#;
/// let map = quake_infostring::parse_key_values(info);
/// assert_eq!(map.get("maxfps"), Some(&"77".to_string()));
/// assert_eq!(map.get("matchtag"), Some(&"kombat".to_string()));
/// assert_eq!(map.get("missing"), None);
/// ```
pub fn parse_key_values(info: &str) -> HashMap<String, String> {
    let info_ = sanitize_kv(info);
    let mut iter = info_
        .trim_matches(KV_DELIMITER)
        .split(KV_DELIMITER)
        .map(|v| v.to_string());
    let mut result = HashMap::new();

    while let Some(key) = iter.next() {
        if let Some(value) = iter.next() {
            result.insert(key, value);
        }
    }

    result
}

/// Sanitize a key-value info string by cleaning common formatting artifacts.
///
/// - Trims leading and trailing whitespace.
/// - Removes trailing literal `\n` sequences.
/// - Replaces escaped delimiters (`\\`) with a single delimiter (`\`).
/// - Removes all double-quote characters (`"`).
/// - Removes any trailing delimiter characters.
fn sanitize_kv(info: &str) -> String {
    info.trim()
        .trim_end_matches(r#"\n"#)
        .replace(r#"\\"#, &KV_DELIMITER.to_string())
        .replace('"', "")
        .trim_end_matches(KV_DELIMITER)
        .to_string()
}

/// Parse a space-separated string into tokens, preserving quoted substrings as single tokens.
///
/// Tokens are split on spaces unless enclosed in double quotes (`"`).
/// Quoted tokens may contain spaces and are returned without the quotes.
/// Empty quoted tokens (`""`) produce empty string tokens.
///
/// # Examples
///
/// ```
/// let input = r#"qtv 1 "zasadzka Qtv (2)" "2@zasadzka.pl:28000" 2"#;
/// let tokens = quake_infostring::parse_fields(input);
/// assert_eq!(tokens, vec![
///     "qtv".to_string(),
///     "1".to_string(),
///     "zasadzka Qtv (2)".to_string(),
///     "2@zasadzka.pl:28000".to_string(),
///     "2".to_string(),
/// ]);
/// ```
pub fn parse_fields(input: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut token = String::new();
    let mut in_quote = false;

    for c in input.chars() {
        match c {
            '"' => in_quote = !in_quote,
            ' ' if !in_quote => {
                tokens.push(token.clone());
                token.clear();
            }
            _ => token.push(c),
        }
    }

    tokens.push(token);
    tokens
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_parse_key_values() {
        {
            let result = parse_key_values(r#"\maxfps\77"#);
            assert_eq!(result.get("maxfps"), Some(&"77".to_string()));
            assert_eq!(result.get("matchtag"), None);
        }
    }

    #[test]
    fn test_sanitize_kv() {
        assert_eq!(sanitize_kv(r#"\\maxfps\\77"#), r#"\maxfps\77"#); // double slashes
        assert_eq!(sanitize_kv(r#"\maxfps\77\"#), r#"\maxfps\77"#); // trailing slash
        assert_eq!(sanitize_kv(r#" \needpass\1\\\n "#), r#"\needpass\1"#); // whitespace
        assert_eq!(sanitize_kv(r#"\max"fps\77""#), r#"\maxfps\77"#); // quotes
    }

    #[test]
    fn test_parse_fields() {
        assert_eq!(
            parse_fields(r#"qtv 1 "zasadzka Qtv (2)" "2@zasadzka.pl:28000" 2"#),
            vec![
                "qtv".to_string(),
                "1".to_string(),
                "zasadzka Qtv (2)".to_string(),
                "2@zasadzka.pl:28000".to_string(),
                "2".to_string(),
            ]
        );
        assert_eq!(
            parse_fields(r#"24 S 0 667 "[ServeMe]" "" 12 11 "lqwc" """#),
            vec![
                "24".to_string(),
                "S".to_string(),
                "0".to_string(),
                "667".to_string(),
                "[ServeMe]".to_string(),
                "".to_string(),
                "12".to_string(),
                "11".to_string(),
                "lqwc".to_string(),
                "".to_string(),
            ]
        );
    }
}
