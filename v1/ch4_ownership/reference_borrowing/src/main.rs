fn change_with_ref(some_string: &mut String) {
    some_string.push_str(" mutable");
}

fn mutable_references() {
    let mut s = String::from("This is");

    println!("Before change_with_ref() = {}", &s);
    change_with_ref(&mut s);
    println!("After change_with_ref() = {}", &s);

}

// This won't compile
fn multiple_mutable_references_v1() {
    let mut s = String::from("Target Reference");

// This won't compile
//    let r0 = &mut s;
//    let r1 = &mut s;

//    println!("{}, {}", r0, r1);
}

/*
    Proper way of declaring multiple mutable reference
*/
fn multiple_mutable_references_v2() {

    let mut s = String::from("Target References: Example");

    let r0 = &mut s;
    println!("r0 = {}", r0);

    /*
    We can look at this way...

    println!(&mut s){
        ... Do internal println operations here

        r0 ends its scope in here
    }

    */

    let r1 = &mut s;
    println!("r1 = {}", r1);

    // This is no longer valid. The ownership on r0 already ended
    // println!("r0 = {}", r0);

}


/*
    Combining immutable and mutable references
    (THIS WON'T COMPILE)
*/
fn multiple_mutable_references_v3() {
    let mut s = String::from("Mutable VS Immutable References");

    let r0 = &s;        // No problem
    let r1 = &s;        // No problem

    // let r2 = &mut s;    // BIG PROBLEM!

    // THIS WILL NOT COMPILE!
    // println!("{}, {}, and {}", r0, r1, r2);

    // This will compile though
    println!("{} and {}", r0, r1);

}


// Lifespan of borrowing
fn borrowing_lifespan_immutable() {

    let mut s = String::from("You cant borrow references forever.");

    let r0 = &s;
    println!("r0 life's ends here = {}", r0);

    let r1 = &s;
    println!("r1 life's ends here = {}", r1);

    let r2 = &s;
    println!("r2 life's ends here = {}", r2);

    let r3 = &s;
    println!("r3 life's ends here = {}", r3);

    // This still compiles
    println!("This will still compile because r0 is still alive...");
    println!("r0 life's ends here = {}", r0);

}

// This won't compile
// fn dangle() -> &String {
//     let s = String::from("This won't compile");

//     &s
// }

// fn dangling_ref() {
//     let s = dangle();
// }

fn main() {
    mutable_references();

//    This won't compile
//    multiple_mutable_references_v1();

    multiple_mutable_references_v2();

    multiple_mutable_references_v3();

    borrowing_lifespan_immutable();

//  This won't compile
//    dangling_ref();
}

