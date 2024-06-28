### Primitives

Rust provides access to a wide variety of `primitives`. A sample includes:

### [Scalar Types](https://doc.rust-lang.org/rust-by-example/primitives.html#scalar-types)

* Signed integers: `i8`, `i16`, `i32`, `i64`, `i128` and `isize` (pointer size)
* Unsigned integers: `u8`, `u16`, `u32`, `u64`, `u128` and `usize` (pointer size)
* Floating point: `f32`, `f64`
* `char` Unicode scalar values like `'a'`, `'α'` and `'∞'` (4 bytes each)
* `bool` either `true` or `false`
* The unit type `()`, whose only possible value is an empty tuple: `()`

Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

### [Compound Types](https://doc.rust-lang.org/rust-by-example/primitives.html#compound-types)

* Arrays like `[1, 2, 3]`
* Tuples like `(1, true)`

## Project Structure

- src/main.rs - consists of main function and its running
- src/primitives.rs - gives intro to the primitives and how scalar types are described
- src/literals.rs - gives intro to the operators
