#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width_ge(&self, rect_rhs: &Rectangle) -> bool {
        self.width >= rect_rhs.width
    }

    fn height_ge(&self, rect_rhs: &Rectangle) -> bool {
        self.height >= rect_rhs.height
    }

    fn can_hold(&self, rect_rhs: &Rectangle) -> bool {
        self.width_ge(rect_rhs) && self.height_ge(rect_rhs)
    }
}




fn main() {
    
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    let rect_ref = &rect;

    let rect_rhs = Rectangle {
        width: 20,
        height: 50,
    };

    println!("area = {:?}", rect.area());
    println!("area_ref = {:?}", rect_ref.area());

    println!("can_hold = {}", rect_ref.can_hold(&rect_rhs));
}
