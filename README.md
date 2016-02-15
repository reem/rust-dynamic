# dynamic

> A dyanmically typed value with fast downcasting.

## [Documentation](https://crates.fyi/crates/dynamic/0.2.0)

Provides a `Dynamic` type, which contains a dynamically typed value. `Dynamic`
is similar to `Any` from `std::any::Any`, except that downcasting does not
involve any virtual calls since the `TypeId` of the contained value is pre-computed.

## Example

```rust
extern crate dynamic;

use dynamic::Dynamic;

fn main() {
    // Create a new Dynamic value.
    let x = Dynamic::new(100usize);

    // If we try to downcast to the wrong type it will not work.
    assert_eq!(x.downcast_ref::<i32>(), None);

    // If we downcast to the right type, we get access.
    assert_eq!(x.downcast_ref::<usize>(), Some(&100usize));
}
```

## Usage

Use the crates.io repository; add this to your `Cargo.toml` along
with the rest of your dependencies:

```toml
[dependencies]
dynamic = "0.2"
```

## Author

[Jonathan Reem](https://medium.com/@jreem) is the primary author and maintainer of dynamic.

## License

MIT

