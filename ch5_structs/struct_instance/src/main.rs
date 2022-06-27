struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_count: u64, 
}

// This won't compile if you set one as mut
// The entire instance must be mutable
struct InvalidStruct {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64, 
    invalid_field: u64,
}

fn declaring_structs() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("corgi_man123"),
        active: true,
        sign_in_count: 1,
    };

    println!("Declaring Struct User...");
    println!("This is user1.email: {}", user1.email);
    println!("This is user1.username: {}", user1.username);
    println!("This is user1.active: {}", user1.active);
    println!("This is user1.sign_in_count: {}", user1.sign_in_count);

}

fn mutating_structs() {

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("corgi_man123"),
        active: true,
        sign_in_count: 1,
    };

    println!("Before Mutation Struct User...");
    println!("This is user1.email: {}", user1.email);
    println!("This is user1.username: {}", user1.username);
    println!("This is user1.active: {}", user1.active);
    println!("This is user1.sign_in_count: {}", user1.sign_in_count);

    // After Mutation 

    user1.email = String::from("anotheremail@example.com");
    user1.username = String::from("corgi_man123");
    user1.active = false; 
    user1.sign_in_count = 2; 

    println!("After Mutation...");
    println!("This is user1.email: {}", user1.email);
    println!("This is user1.username: {}", user1.username);
    println!("This is user1.active: {}", user1.active);
    println!("This is user1.sign_in_count: {}", user1.sign_in_count);
}

fn using_invalid_struct () {
    let my_invalid = InvalidStruct {
        email: String::from("someone@example.com"),
        username: String::from("corgi_man123"),
        active: true,
        sign_in_count: 1,
        invalid_field: 69,
    };

    println!("Using The so-called invalid struct...");
    println!("This is my_invalid.email: {}", my_invalid.email);
    println!("This is my_invalid.username: {}", my_invalid.username);
    println!("This is my_invalid.active: {}", my_invalid.active);
    println!("This is my_invalid.sign_in_count: {}", my_invalid.sign_in_count);

}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn field_init_struct () {

    let user = String::from("cat_man420");
    let email = String::from("cat_man420@gmail.com");

    let user_info = build_user(email, user);

    println!("Using the Short-hand metric...");
    println!("This is user_info.email: {}", user_info.email);
    println!("This is user_info.username: {}", user_info.username);
    println!("This is user_info.active: {}", user_info.active);
    println!("This is user_info.sign_in_count: {}", user_info.sign_in_count);

}

fn struct_update_syntax() {
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

    println!("Using User2 Struct Update Syntax...");
    println!("This is user2.email: {}", user2.email);
    println!("This is user2.username: {}", user2.username);
    println!("This is user2.active: {}", user2.active);
    println!("This is user2.sign_in_count: {}", user2.sign_in_count);

    println!("Warning user1 is no longer active: remember the move semantics for heap allocated variables...");

}

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

// Unit-like Structs Without any field
struct AlwaysEqual;

fn unit_like_struct() {
    println!("Using unit_like_struct()");
    let subject = AlwaysEqual;
} 


// Ownership of Struct Data

fn main() {
    declaring_structs();
    mutating_structs(); 
    using_invalid_struct();
    field_init_struct();
    struct_update_syntax();

    use_tuple_structs();
    unit_like_struct();
}
