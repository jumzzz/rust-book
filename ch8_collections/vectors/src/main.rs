
#[allow(dead_code)]
fn simple_vectors() {
    let mut v0: Vec<i32> = Vec::new();
    let mut v1 = vec![1,2,3];

    println!("v0 = {:?}", v0);
    println!("v1 = {:?}", v1);

    println!("Updating v0:");
    v0.push(5);
    v0.push(5);
    v0.push(5);

    println!("Updating v1:");
    v1.push(6);
    v1.push(6);
    v1.push(6);
    
    println!("After update: ");
    println!("v0 = {:?}", v0);
    println!("v1 = {:?}", v1);

    println!("Reading v1 in for loop");

    for val_ in &v1 {
        println!("val_ = {}", val_);
    }

    let v1_len: usize = v1.len();

    println!("Reading v1 in while loop");

    let mut i: usize = 0;
    
    while i < v1_len {
        println!("v1[{}] = {}", i, &v1[i]);
        i += 1;
    }

}

#[allow(dead_code)]
fn reading_elements_vector() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

}

#[allow(dead_code)]
fn read_out_of_bounds() {
    let v = vec![1, 2, 3, 4, 5];

    // let dne = &v[100]
    let dne = v.get(100);

    println!("dne = {:?}", dne);
}


#[allow(dead_code)]
fn immutable_mutable_borrows() {
    let mut v = vec![1, 2, 3 , 4 ,5];

    let first = &v[0];

//    v.push(6);

    println!("first = {}", first);
}

fn modify_vect() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("v = {:?}", v);

    for i in &mut v {
        *i += 1;
    }

    println!("v = {:?}", v);

}


fn main() {
    reading_elements_vector();
    read_out_of_bounds();
    immutable_mutable_borrows();
    modify_vect();

}
