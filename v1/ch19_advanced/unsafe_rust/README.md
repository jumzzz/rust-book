## Unsafe Superpowers

### Five Features Unsafe Rust Gives you
- Dereference a raw pointer
- Call an unsafe function or method
- Access or modify a mutable static variable
- Implement an unsafe trait
- Access Fields of `union`s

### Important Note
- `unsafe` doesn't turn off the borrow checker or disable other safety checks
- `unsafe` only gives you access to these five features that are then not checked by the compiler for memory safety. You'll still get some degree of safety inside of an unsafe block.
- `unsafe` ensure the code inside an `unsafe` block will access memory in a valid way.
- It lets you to isolate `unsafe` code better since you annotated them as `unsafe`