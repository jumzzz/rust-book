use std::thread;
use std::time::Duration;


#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[allow(dead_code)]
fn inventory_sample() {

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The User with Preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );
}

#[allow(dead_code)]
fn sample_closure_v2() {
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating Slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    
    println!("{}" , expensive_closure(32u32));
}

#[allow(dead_code)]
fn sample_closure_v3() {
    let add_one_v1 = |x: u32| -> u32 {x + 1};
    let add_one_v2 = |x|      -> u32 {x + 1};
    let add_one_v3 = |x|              x + 1 ;

    println!("add_one_v1 = {}", add_one_v1(101u32));
    println!("add_one_v2 = {}", add_one_v2(102u32));
    println!("add_one_v3 = {}", add_one_v3(103u32));
}

#[allow(dead_code)]
fn sample_closure_v4() {
    let sample = |x| x;

    // The compiler will infer the type x as string first
    let s = sample(String::from("hello"));
    let n = sample(String::from("yes"));

    // If you attempt to do this then this won't compile
    // Since sample = |x| x; was already inferred as 
    // |x: String| -> String {x}
    // let m = sample(100u32);

    println!("s = {}", s);
    println!("n = {}", n);
}


// Sample borrowing
#[allow(dead_code)]
fn sample_closure_v5() {
    let list = vec![1, 2, 3];
    println!("Before defining the closure = {:?}", list);
    let only_borrows = || println!("From closure: {:?}", list);
    
    println!("Before calling the closure = {:?}", list);
    only_borrows();
    
    println!("After calling the closure = {:?}", list);
}

// Sample Borrowing with Mutable Reference
#[allow(dead_code)]
fn sample_closure_v6() {

    let mut list = vec![1, 2, 3];
    println!("Before defining the closure = {:?}", list);
    
    let mut borrows_mutably = || list.push(7); 
    borrows_mutably();
    
    println!("After calling the closure = {:?}", list);

}

// With threads
#[allow(dead_code)]
fn sample_closure_v7() {
    let list = vec![1, 2, 3];
    println!("[{:?}] Before defining closure: {:?}", thread::current().id(), list);

    thread::spawn(move || println!("[{:?}] From Other thread = {:?}", thread::current().id(), list))
        .join()
        .unwrap();
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn sample_closure_v8() {
    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
    ];

    list.sort_by_key(|r| r.height);
    println!("{:#?}", list);
}

fn sample_closure_v9() {

    let mut list = [
        Rectangle { width: 10, height: 1},
        Rectangle { width: 3, height: 5},
        Rectangle { width: 7, height: 12},
    ];

    let mut num_ops: u32 = 0;

    list.sort_by_key(|r| {
        num_ops += 1;   
        r.width
    });

    println!("num_ops = {}", num_ops);
    println!("{:#?}", list);

}


fn main() {
    sample_closure_v9();
}
