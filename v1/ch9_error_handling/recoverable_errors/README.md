# Recoverable Errors
- Most errors aren't serious enough to require the program to stop entirely. Sometimes, when a function fails, it's for a reason that you can easily interpret and respond to.
- We can use `Result<T, E>` type where it's defined as

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```