fn takes_ownership(some_string: &String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: &i32) {
    println!("{}", some_integer);
}

fn makes_copy_again(some_integer: i32) {

    println!("{}", some_integer);
}


fn function_ownership_v4() {
    let x0 = 30_i32;

    println!("Invoking function_ownership_v4()");
    makes_copy_again(x0);
    makes_copy_again(x0);
    makes_copy_again(x0);
    makes_copy_again(x0);
    makes_copy_again(x0);
}

/*
   It's important to use reference instead of explicit assignment because
   values are being moved when you do assignment like let y = x

*/
fn function_ownership_v1() {

    println!("Invoking function_ownership_v1()");
    let s = String::from("hello");
    takes_ownership(&s);

    let x = 5;
    makes_copy(&x);

    println!("s variable {}", s);
    println!("x variable {}", x);

}

fn function_ownership_v2() {

    println!("Invoking function_ownership_v2()");
    let x0 = 10;
    let x1 = x0;
    let x2 = x0;
    let x3 = x0;
    let x4 = x0;

    println!("x0 = {}", x0);
    println!("x1 = {}", x1);
    println!("x2 = {}", x2);
    println!("x3 = {}", x3);
    println!("x4 = {}", x4);
}

/* In this case, you cannot repetively do the ff

    let x0 = String::from("hello");
    let x1 = x0;
    let x2 = x0;
    let x3 = x0;
    let x4 = x0;

    However, you can do this

    let x0 = String::from("hello");
    let x1 = &x0;
    let x2 = &x0;
    let x3 = &x0;
    let x4 = &x0;


*/

fn function_ownership_v3() {

    println!("Invoking function_ownership_v3()");
    let x0 = String::from("hello");
    let x1 = &x0;
    let x2 = &x0;
    let x3 = &x0;
    let x4 = &x0;

    println!("x0 = {}", x0);
    println!("x1 = {}", x1);
    println!("x2 = {}", x2);
    println!("x3 = {}", x3);
    println!("x4 = {}", x4);
}



// Allowing copy, clone
#[derive(Copy, Clone)]
struct Point { x: i32, y: i32 }

fn copy_values() {

    let mut p1 = Point{ x: -1, y: 2 };
    let p2 = p1;
    
    println!("Invoking copy_values()");
    println!("p1: {:p}", &p1);
    println!("p2: {:p}", &p2);

}

/*
    Without the need to invoke clone or the Copy trait annotation, we can copy integers since they
    have a known size at compile time and are stored entirely on the 
    stack, so copies of the actual values are quick to make.

*/
fn variables_copy() {
    let x = 5;
    let y = x;

    println!("Invoking variables_copy()");
    println!("pointer of x = {:p}", &x);
    println!("pointer of y = {:p}", &y);
}

fn string_copy() {
    let s1 = String::from("hello");
    let s2 = s1;

    // Move occurs in let s2 = s1;
    // The s1 goes out of scope and no longer valid

    println!("Invoking string_copy()");

    //This results to a compile-time error
    //println!("pointer of s1 = {:p}", s1.as_ptr());
    println!("pointer of s2 = {:p}", s2.as_ptr());

}

fn move_string(some_str: String) -> String {
    some_str
}

fn return_ownership_str() {
    let s1 = String::from("test");
    let s2 = move_string(s1);

    println!("invoking return_ownership()");
    println!("s1 is no longer valid");
    println!("s2 = {:p}", &s2);
}

fn copy_i32(some_i32: i32) -> i32 {
    some_i32
}

fn return_ownership_i32() {
    let x0 = 69_i32;

    let x1 = copy_i32(x0);
    let x2 = copy_i32(x0);
    let x3 = copy_i32(x0);
    let x4 = copy_i32(x0);
    let x5 = copy_i32(x0);

    println!("Invoking return_ownership_i32()");
    println!("Address x0 = {:p}", &x0);
    println!("Address x1 = {:p}", &x1);
    println!("Address x2 = {:p}", &x2);
    println!("Address x3 = {:p}", &x3);
    println!("Address x4 = {:p}", &x4);
    println!("Address x5 = {:p}", &x5);


    println!("Checking the validity of x0 again = {}", x0);

}




fn main() {
    variables_copy();
    string_copy();
    copy_values();

    function_ownership_v1();
    function_ownership_v2();
    function_ownership_v3();
    function_ownership_v4();

    return_ownership_str();
    return_ownership_i32();
}
