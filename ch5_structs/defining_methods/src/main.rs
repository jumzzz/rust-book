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

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated Functions without self
    // Without self as first parameter, it cannot be called as method
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// This is also allowed
impl Rectangle {
    fn perimeter(&self) -> u32 {
        self.width + self.height
    }
}

fn impl_demo_v1() {

    let rect0 = Rectangle{
        width: 30,
        height: 50,
    };

    println!("Basic Implementation of impl in Structs");

    println!("The area of rectangle = {}", rect0.area());
    println!("rect0.nonzero_width = {}", rect0.nonzero_width());
    println!("rect0.nonzero_height = {}", rect0.nonzero_height());

    println!("Displaying rectangle struct = {:?}", rect0);

    let ref_rect0 = &rect0;

    println!("Using References...");
    println!("(&ref_rect0).area() = {}", (&ref_rect0).area());
    println!("(&ref_rect0).nonzero_width() = {}", (&ref_rect0).nonzero_width());
    println!("(&ref_rect0).nonzero_height() = {}", (&ref_rect0).nonzero_height());

    println!("ref_rect0 struct = {:?}", &ref_rect0);

}

fn impl_demo_v2() {
    println!("Implementation by comparing it with other struct...");
    
    let rect0 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect1 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 60,
        height: 45,
    };

    
    println!("Can rect0 hold rect1? = {}", rect0.can_hold(&rect1));
    println!("Can rect0 hold rect2? = {}", rect0.can_hold(&rect2));

}

fn impl_demo_v3() {
    println!("Implementing associated functions w/o self");
    let square = Rectangle::square(5);
    println!("square results = {:?}", square);
}

fn impl_demo_v4() {
    println!("Using another blocks of impl");
    let rect0 = Rectangle {
        width: 10,
        height: 10
    };

    println!("rect0.perimeter() = {}", rect0.perimeter());
}

fn main() {
    impl_demo_v1();
    impl_demo_v2();
    impl_demo_v3();
    impl_demo_v4();
}

