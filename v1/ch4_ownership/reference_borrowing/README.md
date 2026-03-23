# Reference and Borrowing

## References
- Allows you to perform certain operations without ending the **scope** of a particular value
- It can be declared as `&target_variable`

## Borrowing
- The action of creating a reference is called **Borrowing**
- In analogy, if a person owns something, you can borrow it. When you're done, you can give it back -- you don't own it.


## Mutable References

If we to apply changes to a particular variable with references, we can do this

```rust

fn mutable_ref_func(some_datatype: &mut DataType) {
    // Do changes here...
}

// Here are some declarations
let mut data_type: DataType = some_declaration();
mutable_ref_func(&mut data_type);

```

Here's an actual example

```rust
fn change_with_ref(some_string: &mut String) {
    some_string.push_str(" mutable");
}

fn mutable_references() {
    let mut s = String::from("This is");

    println!("Before change_with_ref() = {}", &s);
    change_with_ref(&mut s);
    println!("After change_with_ref() = {}", &s);

}

fn main() {
    mutable_references();
}

```
This results with
```shell
cargo run
...
Compiling reference_borrowing v0.1.0 (/home/jums/hardcore_projects/rust/projects/rust-book/ch4_ownership/reference_borrowing)
Finished dev [unoptimized + debuginfo] target(s) in 0.60s
    Running `target/debug/reference_borrowing`
Before change_with_ref() = This is
After change_with_ref() = This is mutable

```
- This sounds strange, but the benefits are compelling because it allows the compiler to **detect data race condition at compile-time**! 
  
### Note on Data Race/Race Conditions
A Data Race condition happens when three behavior occurs
- Two or more pointers access the same data at the same time.
- At least one of the pointers is being used to write to the data.
- There's no mechanism being used to synchronize access to the data.

## Dangling Pointers
- The Rust compiler guarantees that there's no Dangling pointers
- By observation, you cannot borrow to a variable where it's scope has ended
  
```rust

fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
// This won't compile
```
- Lifetimes will be discussed on chapter 10.

## The Rules of References

Let’s recap what we’ve discussed about references:

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

Next, we’ll look at a different kind of reference: slices.


## Question

How does Rust tracks multiple borrowing?

By the looks of it: 
- You can declare multiple immutable references at a single time but cannot combine it with a single mutable reference
- A mutable reference must exist until the span of its borrowing ends.