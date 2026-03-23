## Use Case for Structs

- Structs allows you to encapsulate data in some sort of Key-Value store
- It's useful on cases where you need to encapsulate data
- Structs can also provide a more intuitive grasp on data which reduces ambiguity in describing your data


## Printing Structs
In order to print `struct`s, you can use the trait attribute `#[derive(Debug)]`

**Example**
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

println!("Displaying struct Rectangle with derived traits: {:?}", &rectangle);
println!("Using dbg! macro");

dbg!(&rectangle);
```

Which results to
```shell
Displaying struct Rectangle with derived traits: Rectangle { width: 20, height: 20 }
Using dbg! macro
[src/main.rs:48] &rectangle = Rectangle {
    width: 20,
    height: 20,
}
```

**Source:** https://doc.rust-lang.org/book/ch05-02-example-structs.html