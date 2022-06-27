/*
    In this section, we will demonstrate why we would want to use structs
*/

// No encapsulation
// More ambiguous
fn area_naive(width: u32, height: u32) -> u32 {
    width * height
}

// This encapsulates the dimensions but indexing tuples can be ambiguous
fn area_tuple(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// This provide lesser ambiguity and more readability
// Adding attributes #[derive(Debug)] for prints

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height    
}

fn main() {

    let naive_area = area_naive(20, 20);
    let tuple_area = area_tuple((20,20));

    let rectangle = Rectangle {
        width: 20,
        height: 20
    };

    let struct_area = area_struct(&rectangle);

    println!("Area naive = {}", naive_area);
    println!("Area tuple = {}", tuple_area);
    println!("Area struct = {}", struct_area);

    println!("Displaying struct Rectangle with derived traits: {:?}", &rectangle);
    println!("Using dbg! macro");

    dbg!(&rectangle);
}
