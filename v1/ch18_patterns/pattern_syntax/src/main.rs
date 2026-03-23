fn matching_literals() {
    let x = 9;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        4 => println!("four"),
        5 => println!("five"),
        _ => println!("idk"),
    }
}

fn match_named_variables() {
    let x = Some(5);
    let y = 10;
    
    match x {
        Some(5) => println!("Some 5!!!"),
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end x = {:?}, y = {y}", x);

}

fn multi_patterns() {
    let x = 4;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("threee!"),
        _ => println!("anythinggggggg"),
    }
}

fn range_patterns() {
    let x = 4;

    match x {
        1..=5 => println!("1-5"),
        6..=10 => println!("6-10"),
        11..=15 => println!("11-15"),
        16..=20 => println!("16-20"),
        _ => println!("Greaterrrrr"),
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn destructure_structs() {
    let p = Point {x: 7, y: 7};

    let Point {x: a, y: b} = p;

    // assert_eq!(0, a);
    // assert_eq!(7, b);

    println!("p = {:?}", p);    
    println!("x = {} y = {}", a, b);

    match p {
        Point {x,  y:0 } => println!("On x axis at {}", x),
        Point {x:0 , y } => println!("On y axis at {}", y),
        Point {x, y } => println!("On neither axis."),

    }
}

enum Movement {
    Forward,
    Backward,
    Left,
    Right,
}

fn match_enums() {
    let next_move = Movement::Backward;

    match next_move {
        Movement::Forward => { println!("Forward!") }
        Movement::Backward => { println!("Backward!") }
        Movement::Left => { println!("Left!") }
        Movement::Right => { println!("Right!") }
    }
}

fn ignore_parts() {
    let numbers = (1,2,3,4,5);

    match numbers {
        (f0, _, _,_, f4) => { println!("{f0}, {f4}") }
    }

    let origin = Point {x: 0, y:32};

    match origin {
        Point {x, y} => println!("y = {y}"),
    }
}

fn using_bindings() {
    let pt = Point {x: 7, y: 9,};

    match pt {
        Point { x: x_val @ 3..=6, y: y_val @ 3..=6, } => println!("yes {} {}", x_val, y_val),
        Point { x: x_val @ 7..=9, y: y_val @ 7..=9, } => println!("hoho {} {}", x_val, y_val),
        _ => println!("nooooo"),
    }
}

fn main() {
    // match_named_variables();
    // matching_literals();
    // multi_patterns();
    // range_patterns();
    // destructure_structs();
    // match_enums();

    // ignore_parts();
    using_bindings();
}
