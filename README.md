# enum2pos

[<img alt="github" src="https://img.shields.io/badge/github-matthewjberger/enum2pos-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/matthewjberger/enum2pos)
[<img alt="crates.io" src="https://img.shields.io/crates/v/enum2pos.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/enum2pos)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-enum2pos-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/enum2pos)

enum2pos is a rust derive macro that creates a Display impl for enums. 
This is useful for strongly typing composable sets of strings.

## Usage

Add this to your `Cargo.toml`:

```toml
enum2pos = "0.1.2"
```

Example:

```rust
use enum2pos::EnumStr;

#[derive(EnumStr)]
enum Object {
    Generic(String),

    #[enum2pos("Color: {}. Shape: {}.")]
    Complex(Color, Shape),
}

#[derive(EnumStr)]
enum Color {
    #[enum2pos("Burgundy")]
    Red,
    Green,
}

#[derive(EnumStr)]
enum Shape {
    Circle,
}

fn main() {
    assert_eq!(Color::Green.to_string(), "Green");

    assert_eq!(Color::Red.to_string(), "Burgundy");

    assert_eq!(Object::Generic("Hello!".to_string()).to_string(), "Hello!");

    assert_eq!(
        Object::Complex(Color::Green, Shape::Circle).to_string(),
        "Color: Green. Shape: Circle."
    );
}
```
