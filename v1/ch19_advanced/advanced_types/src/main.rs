use std::collections::HashMap;

fn type_aliasing() {
    type Kilometers = i32;
    type People = HashMap<i32, String>;

    let people_list = People::from([
        (42069, String::from("Bob Lazar")),
        (11111, String::from("Melon Tusk")),
    ]);
    
    println!("people_list = {:?}", people_list);
}

fn main() {
    type_aliasing();
}
