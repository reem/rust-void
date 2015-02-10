# Void

> The uninhabited void type for use in statically impossible cases.

## Overview

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
void = "*"
```

## Author

[Jonathan Reem](https://medium.com/@jreem) is the primary author and maintainer of void.

## License

MIT

