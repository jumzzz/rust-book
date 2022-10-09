
fn check_lifetime<'a>(x: &'a str,y: &'a str, z: &str) -> &'a str {
    if x.len() > y.len() + z.len() {
        println!("First Condition");
        x
    }
    else if x.len() > y.len() {
        println!("Second Condition");
        x
    }
    else {
        println!("Third Condition");
        y
    }
}


// This won't compile
// fn check_lifetime(x: &str,y: &str, z: &str) -> &str {
//     if x.len() > y.len() + z.len() {
//         println!("First Condition");
//         x
//     }
//     else if x.len() > y.len() {
//         println!("Second Condition");
//         x
//     }
//     else {
//         println!("Third Condition");
//         y
//     }
// }

fn main() {
    let s0 = "aaa-bbb";
    let s1 = "xyz";
    let s2 = "zzz";

    let result = check_lifetime(s0,s1, s2);
    println!("result = {}", result);
}
