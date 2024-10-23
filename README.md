# enum2pos

[<img alt="github" src="https://img.shields.io/badge/github-matthewjberger/enum2pos-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/matthewjberger/enum2pos)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enum2pos.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/enum2pos)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-enum2pos-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/enum2pos)

enum2pos is a rust derive macro for enums that generates `from_index(usize, Vec<String>) -> Option<Self>` and
`to_index()` methods for converting between an variants and
their position within the enum declaration (similar to an index).

## Usage

Add this to your `Cargo.toml`:

```toml
enum2pos = "0.1.2"
```

Example:

```rust
use enum2pos::EnumIndex;

#[derive(EnumIndex, PartialEq, Debug)]
enum SampleEnum {
    Unit,
    Unnamed(i32, String),
}

#[test]
fn to_index() {
    let unit = SampleEnum::Unit;
    let unnamed = SampleEnum::Unnamed(42, String::from("test"));

    assert_eq!(unit.to_index(), 0);
    assert_eq!(unnamed.to_index(), 1);
}

#[test]
fn from_index_unit() {
    let index = 0;
    let args: Vec<String> = vec![];
    let expected = Some(SampleEnum::Unit);

    assert_eq!(SampleEnum::from_index(index, &args), expected);
}

#[test]
fn from_index_unnamed() {
    let index = 1;
    let args = vec!["42".to_string(), "test".to_string()];
    let expected = Some(SampleEnum::Unnamed(42, String::from("test")));

    assert_eq!(SampleEnum::from_index(index, &args), expected);
}

#[test]
fn from_index_invalid() {
    let index = 2;
    let args: Vec<String> = vec![];

    assert_eq!(SampleEnum::from_index(index, &args), None);
}
```
