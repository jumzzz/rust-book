# Unrecoverable Errors
- By default, when panic occurs, the program starts `unwinding`, which means Rust walks back up the stack and cleans up the data from each function it encounters.
- This can cause performance related issues
- Rust, therefore, allows you to choose the alternative and immediately aborting, which ends the program without cleaning up.
- You can do this with the following in your `Cargo.toml` file

```
[profile.release]
panic = 'abort'
```
### Using Backtrace
- You can use the following command

```
RUST_BACKTRACE=1 cargo run
```

