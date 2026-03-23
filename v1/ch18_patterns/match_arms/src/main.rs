fn multi_colors() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = true;
    let age: Result<u8,_> = "xx".parse();

    if let Some(color) = favorite_color {
        println!("Favorite color is Some({color})");
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("is_tuesday = {is_tuesday}");
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn enum_loops() {
    let v = vec!['a', 'b', 'c'];
    let v_len = v.len();
    
    for (index, value) in v.iter().enumerate() {
        println!("[{}/{}] Current Value = {}", index + 1, v_len, value);
    }
}

fn print_tuples(&(x,y,z): &(i32, i32, i32)) {
    println!("coordinates = ({},{},{})", x,y,z);

}

fn let_tuples() {
    let (x0, y0, z0) = (1,1,1);
    let (x1, y1, z1) = (2,2,2);
    let (x2, y2, z2) = (3,3,3);
    let (x3, y3, z3) = (4,4,4);

    println!("(x0,y0,z0) = ({},{},{})", x0,y0,z0);
    println!("(x1,y1,z1) = ({},{},{})", x1,y1,z1);
    println!("(x2,y2,z2) = ({},{},{})", x2,y2,z2);
    println!("(x3,y3,z3) = ({},{},{})", x3,y3,z3);

    let pt0 = (1,1,1);
    let pt1 = (2,2,2);
    let pt2 = (3,3,3);
    let pt3 = (4,4,4);

    print_tuples(&pt0);
    print_tuples(&pt1);
    print_tuples(&pt2);
    print_tuples(&pt3);

}




fn main() {
    let_tuples();
}
