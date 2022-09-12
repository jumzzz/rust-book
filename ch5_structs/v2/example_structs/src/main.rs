fn main() {
    let width1 = 30;
    let height1 = 50;

    let rect1 = (width1, height1);
    let rect2 = Rectangle {
        width: width1,
        height: height1,
    };

    println!(
        "The area of the rectangle is {} square pixels -- Vanilla ver",
        area(width1, height1)
    );

    println!(
        "The area of the rectangle is {} square pixels. -- Tuple ver",
        area_tuples(rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels. -- Struct ver",
        area_struct(&rect2)
    );

    println!(
        "Printing the values of the rectangle. {:?}", rect2
    );

    let scale = 2;

    let rect_dbg = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect_dbg);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area(width: u32, height: u32) -> u32 {
    width * height 
}

fn area_tuples(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area_struct(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

