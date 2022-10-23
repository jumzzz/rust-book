
// Just playing around with the 
// memory location of the box
fn box_sample_v1() {
    let x = 5;
    println!("x_ptr (Before boxing) = {:p}", &x);
    let y = Box::new(x);
    println!("y_ptr (Before boxing) = {:p}", &*y);
}


// This is the key Deref trait
// You need to have it inherited to your MyBox

use std::ops::Deref;

struct MyBox<T>(T);


impl<T> Deref for MyBox<T> {
    type Target = T;

    // Makes Rust runs *(y.deref()) behind the scenes
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}



impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn compare_box_mem_loc() {
    let x = 5;
    let y = Box::new(5);
    let z = MyBox::new(String::from("Hello This is some string"));
    let vv = vec![1,2,3,4,5];

    println!("x_ptr = {:p}", &x);
    println!("y_ptr = {:p}", &*y);
    println!("z_ptr = {:p}", &**z);
    println!("vv_ptr = {:p}", &*vv);
}

fn main() {

    compare_box_mem_loc();


}
