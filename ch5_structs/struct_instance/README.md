# Instantiating Struct in Rust

## What is Structs

It allows you to declare multiple datatypes in a single instance

**Example:**
```rust
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64
}

```

## Mutability and Immutability of Structs
- Structs are pretty consistent with Rust's rules when it comes to mutability/immutability
- You cannot make a single field in Rust solely immutable though.

In order to declare a `struct` as mutable, you have to declare the single `struct` instance as mut

```rust
struct User{...}

let mut user = User{
    username: String::from("jumzzz"),
    email: String::from("jumzzz@myemail.com"),
    active: true,
    sign_in_count: 420,
};

// This allows you to make modifications
user.username = String::from("new_user");
user.email = String::from("new_email@example.com");
user.active = true;
user.sign_in_count = 204; 

```

## Shorthand Declaration of structs

### Using Field Init Structs

You can declare field init structs like this
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Which lets you easily
let user = String::from("cat_man420");
let email = String::from("cat_man420@gmail.com");
let user_info = build_user(email, user);

```

### Using Struct Update Syntax

You can directly copy from one struct to another (i.e. `user1` to `user2`) with

```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("corgi_man123"),
    active: true,
    sign_in_count: 1,
};

let user2 = User{
    email: String::from("damn_another_user@example.com"),
    ..user1
};

```
### Word of warning with using Update syntax
- The move semantics remains valid with Update Syntax. Be aware of the variables
that tends to move when assigned using Update Syntax.


## Using Tuple Structs

You can create unnamed fields with `struct` which is similar to Tuple

```rust
// Using Tuple Structs;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn use_tuple_structs() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("Using use_tuple_structs");
    println!("black = ({}, {}, {})", black.0, black.1, black.2);
    println!("origin = ({}, {}, {})", origin.0, origin.1, origin.2);
}
```

## On Ownership of Struct Data

Fields on structs can have references on their respective fields but doing so requires *lifetimes* which will be discussed on Chapter 10.


**Source:** https://doc.rust-lang.org/book/ch05-01-defining-structs.html
