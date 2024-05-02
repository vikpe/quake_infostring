use crate::DELIMITER;

pub fn clean(serverinfo: &str) -> String {
    serverinfo
        .trim()
        .trim_end_matches(r#"\n"#)
        .replace(r#"\\"#, &DELIMITER.to_string())
        .replace('"', "")
        .trim_end_matches(DELIMITER)
        .to_string()
}


#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;

    #[test]
    fn test_clean() {
        assert_eq!(clean(r#"\\maxfps\\77"#), r#"\maxfps\77"#); // double slashes
        assert_eq!(clean(r#"\maxfps\77\"#), r#"\maxfps\77"#); // trailing slash
        assert_eq!(clean(r#" \needpass\1\\\n "#), r#"\needpass\1"#); // whitespace
        assert_eq!(clean(r#"\max"fps\77""#), r#"\maxfps\77"#); // quotes
    }
}
