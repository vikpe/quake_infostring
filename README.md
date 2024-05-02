# quake_infostring [![Test](https://github.com/vikpe/quake_infostring/actions/workflows/test.yml/badge.svg?branch=main)](https://github.com/vikpe/quake_infostring/actions/workflows/test.yml) [![crates](https://img.shields.io/crates/v/quake_infostring)](https://crates.io/crates/quake_infostring) [![docs.rs](https://img.shields.io/docsrs/quake_infostring)](https://docs.rs/quake_infostring/)

> Parse QuakeWorld info strings

## Usage

```rust
let info_str = r#"\maxfps\77\matchtag\kombat"#;
let info = quake_infostring::to_hashmap(&info_str);

assert_eq!(info.get("maxfps"), Some(&"77".to_string()));
assert_eq!(info.get("matchtag"), Some(&"kombat".to_string()));
assert_eq!(info.get("MISSING_KEY"), None);
```

## See also

* [quake_serverinfo](https://github.com/vikpe/quake_serverinfo) - Parse QuakeWorld serverinfo strings
