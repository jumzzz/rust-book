# Slices

Slices allows you to take the subset of the iteratable references

It starts with a general convention

```rust
let source = "target source";
&source[start_index..end_index];
```

### Example:

```rust
let s = "hello world";
let len_s = s.len();

println!("{}", &s[0..len_s]);   // hello world
println!("{}", &s[0..5]);       // hello
println!("{}", &s[5..]);        // world

```

**Source:** https://doc.rust-lang.org/book/ch04-03-slices.html