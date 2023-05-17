# c-enum

![ci badge]
[![crates.io badge]](https://crates.io/crates/c-enum)
[![docs.rs badge]](https://docs.rs/c-enum)

[ci badge]: https://img.shields.io/github/actions/workflow/status/phantomical/c-enum/dispatch.yml?branch=main&style=flat-square
[docs.rs badge]: https://img.shields.io/docsrs/c-enum?style=flat-square
[crates.io badge]: https://img.shields.io/crates/v/c-enum?style=flat-square

A rust macro for easily defining structs that act like C enums.

- [Documentation](https://docs.rs/c-enum)
- [Release Notes](https://github.com/phantomical/c-enum/blob/main/CHANGELOG.md)

# Example
```rust
use c_enum::c_enum;

c_enum! {
    #[derive(Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
    pub enum Variant: u32 {
        A,
        B,
        C = 5,
    }
}

fn main() {
    let v1 = Variant::A;
    let v2 = Variant::from(77);

    match v1 {
        Variant::A => println!("A"),   // named values...
        Variant::B => println!("B"),
        Variant(77) => println!("77"), // values without named variants also work
        _ => println!("other value"),
    }
}
```
