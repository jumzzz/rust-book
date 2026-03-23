fn largest<T: PartialOrd>(list: &[T]) -> &T {
    
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}


fn find_largest() {

    let v0 = vec![34, 50, 25, 100, 65];
    let v1 = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    let v0_max = largest(&v0);
    let v1_max = largest(&v1);

    println!("v0_max = {}", v0_max);
    println!("v1_max = {}", v1_max);

    let c0 = vec!['a', 'b', 'c', 'd', 'e'];
    let c1 = vec!['v', 'w', 'x', 'y', 'z'];

    let c0_max = largest(&c0);
    let c1_max = largest(&c1);

    println!("c0_max = {}", c0_max);
    println!("c1_max = {}", c1_max);

}


#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

// Specifying constraints on implementation
impl Point<f32, f32> {
    fn dist(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}



fn init_generic_struct() {
    
    let diff_pt = Point {x: 20.0, y: 20.5};

    println!("diff_pt = {:?}", diff_pt);
    println!("diff_pt.x = {}", diff_pt.x());
    println!("diff_pt.y = {}", diff_pt.y());
    
    println!("diff_pt.dist = {}", diff_pt.dist());
}



fn main() {
//    find_largest();
    init_generic_struct();
}
