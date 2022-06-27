## Methods

This allows you you to associate functions within a struct. This is similar to a Class object in Object Oriented languages.

It starts with an `impl` block and each of the declared `fn` in `impl` block must start with `&self` as an argument. 


### Example

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    
    // All functions defined with impl block are called associated functions
    fn area(&self) -> u32 {
        self.width * self.height
    }
    ...
}
```

## Associated Functions

This is similar to static methods to Object Oriented Languages. It's called Associated functions if the member function doesn't start with self as the argument.

### Example

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    ...
    // Associated Functions without self
    // Without self as first parameter, it cannot be called as method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
```

## Additional note

You can declare multiple blocks of `impl` and this will still work

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    
    // All functions defined with impl block are called associated functions
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn nonzero_width(&self) -> bool {
        self.width > 0
    }

    fn nonzero_height(&self) -> bool {
        self.height > 0
    }
}

// This is also allowed
impl Rectangle {
    fn perimeter(&self) -> u32 {
        self.width + self.height
    }
}

```

**Source:** https://doc.rust-lang.org/book/ch05-03-method-syntax.html