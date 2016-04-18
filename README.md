# Void

> The uninhabited void type for use in statically impossible cases.

## [Documentation](https://crates.fyi/crates/void/1.0.1)

The uninhabited type, `enum Void { }` is useful in dealing with cases you
know to be impossible. For instance, if you are implementing a trait which
allows for error checking, but your case always succeeds, you can mark the
error case or type as `Void`, signaling to the compiler it can never happen.

This crate also comes packed with a few traits offering extension methods to
`Result<T, Void>` and `Result<Void, T>`.

## Usage

Use the crates.io repository; add this to your `Cargo.toml` along
with the rest of your dependencies:

```toml
[dependencies]
void = "1"
```

Then, use `Void` in your crate:

```rust
extern crate void;
use void::Void;
```

## Author

[Jonathan Reem](https://medium.com/@jreem) is the primary author and maintainer of void.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.

