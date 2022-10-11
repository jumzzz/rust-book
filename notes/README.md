# Some Notes

## Things that bothers me

- I've written the following snippet
  
```rust
fn read_file(file_path: &String) -> Vec<&str> {

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let res: Vec<&str> = contents.split("\n").collect();
    res
}
```

- And this return's an error
```bash
cargo run -- smeagol sample_file/sample.txt 
   Compiling minigrep v0.1.0 (/home/jums/hardcore_projects/rust/projects/rust-book/ch12_io_project/minigrep)
error[E0515]: cannot return value referencing local variable `contents`
  --> src/main.rs:10:5
   |
9  |     let res: Vec<&str> = contents.split("\n").collect();
   |                          -------------------- `contents` is borrowed here
10 |     res
   |     ^^^ returns a value referencing data owned by the current function

For more information about this error, try `rustc --explain E0515`.
error: could not compile `minigrep` due to previous error

```