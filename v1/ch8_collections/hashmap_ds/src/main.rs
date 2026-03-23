use std::collections::HashMap;

fn sample_hash_map() {

    let mut scores = HashMap::new();

    let key_blue = String::from("Blue");
    let key_yellow = String::from("Yellow");

    scores.insert(&key_blue, 10);
    scores.insert(&key_yellow, 50);
    
    println!("{:?}", &scores);

    println!("scores[\"{:?}\"] = {:?}", &key_blue, scores.get(&key_blue));
    println!("scores[\"{:?}\"] = {:?}", &key_yellow, scores.get(&key_yellow));

    println!("Iterating hashmap");

    for (key,value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn conditional_entry() {
    println!("conditional_entry = ");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", &scores);

}

fn update_with_pointer() {

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);


}


fn main() {
    sample_hash_map();
    conditional_entry();
    update_with_pointer();

}
