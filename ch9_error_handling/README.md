# Chapter 9: Error Handling

## Two Types of Errors in Rust
- **Recoverable Error**
- **Unrecoverable Error**

### Recoverable Errors
- It wants to reports problem to the user and retry the operation
- One example is *file not found* error

### Unrecoverable Errors
- Always a symptoms of bugs
- One example is out-of-bounds memory access

## Rust doesn't have Exceptions
- Instead, it has the type `Result<T, E>` for recoverable error
- `panic!` macro that stops execution when the program encounters and unrecoverable error.