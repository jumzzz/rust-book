fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_two(x: i32) -> i32 {
    x + 2
}

fn add_something(x: i32, add_func: fn(i32) -> i32) -> i32 {
    add_func(x) 
}

fn func_as_args() {
    println!("Let's use function pointers: ");
    println!("add_one = {}", add_something(3, add_one));
    println!("add_one = {}", add_something(3, add_two));
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point {
            x: x,
            y: y,
        }
    }
}

fn using_closures() {

    let pt = Point::new(3,3);
    let tupl_list: Vec<(u32, u32)> = vec![
        (1, 2),
        (2, 3),
        (3, 4),
        (4, 5),
        (5, 6),
    ];

    let point_list: Vec<Point> = tupl_list.iter().map(
        |pt| Point::new(pt.0, pt.1)).collect();

    println!("Tuple List");
    println!("tupl_list = {:?}", tupl_list);

    println!("Point List");
    println!("point_list = {:?}", point_list);
}


fn main() {
    using_closures();
}
