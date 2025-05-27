# quake_infostring [![Test](https://github.com/vikpe/quake_infostring/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/quake_infostring/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/quake_infostring)](https://crates.io/crates/quake_infostring) [![docs.rs](https://img.shields.io/docsrs/quake_infostring)](https://docs.rs/quake_infostring/)

> Parse QuakeWorld info strings

## Usage

```rust
// parse key values
let info = r#"\maxfps\77\matchtag\kombat"#;
let map = quake_infostring::parse_key_values(info);
assert_eq!(map.get("maxfps"), Some(&"77".to_string()));
assert_eq!(map.get("matchtag"), Some(&"kombat".to_string()));
assert_eq!(map.get("missing"), None);

// parse fields
let input = r#"qtv 1 "zasadzka Qtv (2)" "2@zasadzka.pl:28000" 2"#;
let tokens = quake_infostring::parse_fields(input);
assert_eq!(tokens, vec![
    "qtv".to_string(),
    "1".to_string(),
    "zasadzka Qtv (2)".to_string(),
    "2@zasadzka.pl:28000".to_string(),
    "2".to_string(),
]);
```

## See also

- [quake_serverinfo](https://github.com/vikpe/quake_serverinfo) - Parse QuakeWorld serverinfo strings
