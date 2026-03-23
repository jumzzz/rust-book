fn return_from_loop(v0: i32) -> i32 {
    let mut v1 = v0;
    loop {
        v1 += 1;
        if v1 - v0 == 10 {
            break;
        }
    }
    v1
}

fn main() {
    let v0: i32 = 10;
    let v1 = return_from_loop(v0);
    println!("v0 = {v0}");
    println!("v1 = {v1}");
}
